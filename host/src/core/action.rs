use btleplug::{api::Central as _, platform::PeripheralId};
use anyhow::Result;

use super::State;

#[derive(Debug, Clone)]
pub enum Action {
  StartScan,
  StopScan,
  Select(PeripheralId),
  Connect(PeripheralId),
  Disconnect(PeripheralId),
  Refresh,
  Exit,
}

impl Action {
  pub async fn apply(&self, state: &mut State) -> Result<()> {
    match self {
      Action::StartScan => state.current_adapter.start_scan(Default::default()).await?,
      Action::StopScan => state.current_adapter.stop_scan().await?,
      Action::Select(id) => {
        state.current_peripheral_id = Some(id.clone());
        // state.get_properties().await?;
      },
      Action::Connect(id) => todo!("action: connect {id}"),
      Action::Disconnect(id) => todo!("action: disconnect {id}"),
      Action::Refresh => state.update_peripherals().await?,
      Action::Exit => return Err(anyhow::anyhow!("Exit")),
    }
    Ok(())
  }
}
