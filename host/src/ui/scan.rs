use std::collections::HashMap;

use btleplug::platform::PeripheralId;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
  layout::{Constraint, Layout},
  prelude::{Buffer, Rect},
  text::Line,
  widgets::{Clear, List, ListItem, Paragraph, Widget}
};
use itertools::Itertools;
use anyhow::Result;

use super::{EventHandler, Stateful};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeripheralInfo {
  pub id: PeripheralId,
  pub local_name: String,
  pub details: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScanTab {
  pub data: Vec<PeripheralInfo>,
  pub current: usize,
}

impl ScanTab {
  pub fn new() -> Self {
    Self {
      data: vec![],
      current: 0,
    }
  }

  /// Select the previous email (with wrap around).
  pub fn prev(&mut self) {
    if self.data.is_empty() { return }
    self.current = self.current.saturating_add(self.data.len() - 1) % self.data.len();
  }

  /// Select the next email (with wrap around).
  pub fn next(&mut self) {
    if self.data.is_empty() { return }
    self.current = self.current.saturating_add(1) % self.data.len();
  }
}

impl EventHandler for ScanTab {
  async fn handle_event(&self, stateful: &mut Stateful, event: Event) -> Result<()> {
    match event {
      Event::Key(key) if key.kind == KeyEventKind::Press => {
        match key.code {
          KeyCode::Up => {
            if self.data.is_empty() { return Ok(()) }
            let prev = self.current.saturating_add(self.data.len() - 1) % self.data.len();
            let peripheral_id = self.data[prev].id.clone();
            stateful.action_tx.send(crate::core::action::Action::Select(peripheral_id)).await?;
          },
          KeyCode::Down => {
            if self.data.is_empty() { return Ok(()) }
            let next = self.current.saturating_add(1) % self.data.len();
            let peripheral_id = self.data[next].id.clone();
            stateful.action_tx.send(crate::core::action::Action::Select(peripheral_id)).await?;
          },
          _ => {},
        }
      }
      _ => {},
    }
    Ok(())
  }
}

impl Widget for ScanTab {
  fn render(self, area: Rect, buf: &mut Buffer) {
    Clear.render(area, buf);
    let [list, data] = Layout::horizontal([Constraint::Length(20), Constraint::Min(0)]).areas(area);
    self.render_list(list, buf);
    self.render_detail(data, buf);
  }
}

impl ScanTab {
  fn render_list(&self, area: Rect, buf: &mut Buffer) {
    let [header_area, body_area] = Layout::default()
      .direction(ratatui::layout::Direction::Vertical)
      .constraints([Constraint::Length(1), Constraint::Min(0)])
      .areas(area);
    Paragraph::new("Peripherals").render(header_area, buf);
    let items = self.data.iter().enumerate().map(|(i, peripheral)| {
      let name = if peripheral.local_name.is_empty() {
        peripheral.id.to_string()
      } else {
        peripheral.local_name.clone()
      };
      let indicator = if i == self.current { "> " } else { "  " };
      ListItem::new(Line::from(vec![indicator.into(), name.into()]))
    }).collect::<Vec<_>>();
    List::new(items)
      .render(body_area, buf);
  }

  fn render_detail(&self, area: Rect, buf: &mut Buffer) {
    let peripheral = self.data.get(self.current);
    if let Some(peripheral) = peripheral {
      let details = peripheral.details.iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(k, v)| {
          ListItem::new(Line::from(vec![k.clone().into(), ": ".into(), v.clone().into()]))
        })
        .collect::<Vec<_>>();
      List::new(details)
        .render(area, buf);
    }
  }
}
