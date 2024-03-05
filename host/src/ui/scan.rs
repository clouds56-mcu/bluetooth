use ratatui::{layout::{Constraint, Layout}, widgets::{Clear, Paragraph, Widget}};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Peripheral {
  local_name: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PeripheralTab {
  data: Vec<Peripheral>,
  current: usize,
}

impl PeripheralTab {
  pub fn new() -> Self {
    Self {
      data: vec![Peripheral {
        local_name: "test".to_string(),
      }],
      current: 0,
    }
  }

  /// Select the previous email (with wrap around).
  pub fn prev(&mut self) {
    self.current = self.current.saturating_add(self.data.len() - 1) % self.data.len();
  }

  /// Select the next email (with wrap around).
  pub fn next(&mut self) {
    self.current = self.current.saturating_add(1) % self.data.len();
  }
}

impl Widget for PeripheralTab {
  fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
    Clear.render(area, buf);
    let vertical = Layout::vertical([Constraint::Length(5), Constraint::Min(0)]);
    let [list, data] = vertical.areas(area);
    Paragraph::new("list").render(list, buf);
    Paragraph::new("data").render(data, buf)
    // render_inbox(self.current, inbox, buf);
    // render_email(self.current, email, buf);
  }
}
