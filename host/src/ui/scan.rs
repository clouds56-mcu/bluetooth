use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
  layout::{Constraint, Layout},
  prelude::{Buffer, Rect},
  style::Stylize,
  text::Line,
  widgets::{Clear, List, ListItem, Paragraph, Widget}
};

use super::{EventHandler, Stateful};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PeripheralInfo {
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
  fn handle_event(&self, _stateful: &mut Stateful, event: Event) {
    match event {
      Event::Key(key) if key.kind == KeyEventKind::Press => {
        match key.code {
          // KeyCode::Up => self.prev(),
          // KeyCode::Down => self.next(),
          _ => {},
        }
      }
      _ => {},
    }
  }
}

impl Widget for ScanTab {
  fn render(self, area: Rect, buf: &mut Buffer) {
    Clear.render(area, buf);
    let [list, data] = Layout::horizontal([Constraint::Length(20), Constraint::Min(0)]).areas(area);
    self.render_list(list, buf);
    Paragraph::new("data").white().on_blue().render(data, buf);
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
      let name = peripheral.local_name.as_str();
      let indicator = if i == self.current { "> " } else { "  " };
      ListItem::new(Line::from(vec![indicator.into(), name.into()]))
    }).collect::<Vec<_>>();
    List::new(items)
      .render(body_area, buf);
  }
}
