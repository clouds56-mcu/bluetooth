use anyhow::Result;
use ratatui::{backend::Backend, Terminal};
use tokio::sync::watch::Receiver;

pub mod scan;

pub struct UiState {
  pub scan: scan::ScanTab,
}

impl From<&crate::core::State> for UiState {
  fn from(state: &crate::core::State) -> Self {
    Self {
      scan: scan::ScanTab::from(state),
    }
  }
}

pub struct Ui {
  state_rx: Receiver<UiState>,
}

impl Ui {
  pub fn new(state_rx: Receiver<UiState>) -> Self {
    Self { state_rx }
  }

  pub async fn run(mut self) -> Result<()> {
    let mut terminal = setup()?;
    loop {
      let new_state = self.state_rx.borrow_and_update();

      terminal.draw(|frame| {
        let area = frame.size();
        frame.render_widget(new_state.scan.clone(), area);
      })?;

      if crossterm::event::poll(std::time::Duration::from_millis(16))? {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
          if key.kind == crossterm::event::KeyEventKind::Press && key.code == crossterm::event::KeyCode::Char('q') {
            break;
          }
        }
      }
    }
    finalize()?;
    Ok(())
  }
}

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
