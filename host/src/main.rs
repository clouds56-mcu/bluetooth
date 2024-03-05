use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};

pub mod core;
pub mod ui;

fn main() -> Result<()> {
  let mut terminal = ui::setup()?;
  loop {
    terminal.draw(|frame| {
      let area = frame.size();
      frame.render_widget(
        ui::scan::PeripheralTab::new(),
        area,
      );
    })?;

    if event::poll(std::time::Duration::from_millis(16))? {
      if let event::Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')
        {
          break;
        }
      }
    }
  }
  ui::finalize()?;
  // core::demo()?;
  Ok(())
}
