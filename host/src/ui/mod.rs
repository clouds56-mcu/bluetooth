use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::{backend::Backend, Terminal};
use tokio::sync::{mpsc, watch};

pub mod scan;

#[derive(Debug, Clone)]
pub struct Stateful {
  pub current_tab: usize,
  pub action_tx: mpsc::Sender<crate::core::Action>,
}

impl Stateful {
  pub fn new(action_tx: mpsc::Sender<crate::core::Action>) -> Self {
    Self {
      current_tab: Default::default(),
      action_tx,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Props {
  pub scan: scan::ScanTab,
}

#[allow(async_fn_in_trait)]
pub trait EventHandler {
  async fn handle_event(&self, stateful: &mut Stateful, event: Event) -> Result<()>;
}

impl EventHandler for Props {
  async fn handle_event(&self, stateful: &mut Stateful, event: Event) -> Result<()> {
    self.scan.handle_event(stateful, event).await?;
    Ok(())
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
  props_rx: watch::Receiver<Props>,
}

impl Ui {
  pub fn new(props_rx: watch::Receiver<Props>) -> Self {
    Self { props_rx }
  }

  pub async fn run(mut self, action_tx: mpsc::Sender<crate::core::Action>) -> Result<()> {
    let mut stateful = Stateful::new(action_tx);
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
        new_props.handle_event(&mut stateful, event).await.ok();
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
