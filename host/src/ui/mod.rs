use anyhow::Result;
use ratatui::{backend::Backend, Terminal};

pub mod scan;

pub fn setup() -> Result<Terminal<impl Backend>> {
  initialize_panic_handler();
  crossterm::terminal::enable_raw_mode()?;
  crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
  let terminal = ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stderr()))?;
  Ok(terminal)
}

pub fn finalize() -> Result<()> {
  crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
  crossterm::terminal::disable_raw_mode()?;
  Ok(())
}

pub fn initialize_panic_handler() {
  let original_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
    finalize().unwrap();
    original_hook(panic_info);
  }));
}
