use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};
use tokio::sync::watch::Receiver;

pub mod scan;

#[derive(Debug, Clone, Default)]
pub struct Stateful {
  pub current_tab: usize,
}

#[derive(Debug, Clone)]
pub struct Props {
  pub scan: scan::ScanTab,
}

pub trait EventHandler {
  fn handle_event(&self, stateful: &mut Stateful, event: Event);
}

impl EventHandler for Props {
  fn handle_event(&self, stateful: &mut Stateful, event: Event) {
    self.scan.handle_event(stateful, event);
  }
}

impl From<&crate::core::State> for Props {
  fn from(state: &crate::core::State) -> Self {
    Self {
      scan: scan::ScanTab::from(state),
    }
  }
}

pub struct Ui {
  props_rx: Receiver<Props>,
}

impl Ui {
  pub fn new(props_rx: Receiver<Props>) -> Self {
    Self { props_rx }
  }

  pub async fn run(mut self) -> Result<()> {
    let mut stateful = Stateful::default();
    let mut terminal = setup()?;
    loop {
      let new_props = self.props_rx.borrow_and_update();

      terminal.draw(|frame| {
        let area = frame.size();
        frame.render_widget(new_props.scan.clone(), area);
      })?;

      if crossterm::event::poll(std::time::Duration::from_millis(16))? {
        let event = crossterm::event::read()?;
        if let crossterm::event::Event::Key(event) = event {
          if event.code == KeyCode::Char('q') && event.modifiers.is_empty() {
            break;
          }
        }
        new_props.handle_event(&mut stateful, event);
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
