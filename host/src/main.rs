use anyhow::Result;

pub mod core;
pub mod ui;

#[tokio::main]
async fn main() -> Result<()> {
  let (state, state_rx) = core::StateStore::new()?;
  let ui = ui::Ui::new(state_rx);
  let handle = tokio::spawn(state.run());
  ui.run().await?;
  handle.abort();
  Ok(())
}
