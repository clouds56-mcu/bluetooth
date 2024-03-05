use anyhow::Result;
use btleplug::api::Central;
use crossterm::event::{self, KeyCode, KeyEventKind};

pub mod core;
pub mod ui;

#[tokio::main]
async fn main() -> Result<()> {
  let mut terminal = ui::setup()?;
  let mut state = core::State::new()?;
  state.current_adapter.start_scan(Default::default()).await?;
  loop {
    state.peripherals = state.current_adapter.peripherals().await?;

    let tab = ui::scan::PeripheralTab::from(&state);
    terminal.draw(|frame| {
      let area = frame.size();
      frame.render_widget(tab, area);
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
