use std::collections::HashMap;

use btleplug::api::PeripheralProperties;
use btleplug::api::{Central as _, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral, PeripheralId};
use anyhow::Result;
use tokio::sync::{mpsc, watch};

use crate::ui::scan::PeripheralInfo;
use crate::ui::{self, Props};

use super::action::Action;

pub struct State {
  pub adapters: Vec<Adapter>,
  pub current_adapter: Adapter,
  pub peripherals: Vec<Peripheral>,
  pub current_peripheral_id: Option<PeripheralId>,
  pub peripheral_details: HashMap<PeripheralId, PeripheralInfo>,
}

impl From<&State> for ui::scan::ScanTab {
  fn from(state: &State) -> Self {
    let data = state.peripherals.iter().map(|p| {
      let properties = p.get_properties().unwrap_or_default();
      let details = [
        ("Address", properties.address.to_string()),
        ("Address Type", properties.address_type.map(|i| format!("{:?}", i)).unwrap_or_default()),
        ("Local Name", properties.local_name.unwrap_or_default()),
        ("Tx Power Level", properties.tx_power_level.map(|i| format!("{:?}", i)).unwrap_or_default()),
        ("RSSI", properties.rssi.map(|i| format!("{:?}", i)).unwrap_or_default()),
        ("Manufacturer", format!("{:x?}", properties.manufacturer_data.keys().collect::<Vec<_>>())),
        ("Services", format!("{:x?}", properties.services)),
        ("Class", properties.class.map(|i| i.to_string()).unwrap_or_default()),
      ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
      ui::scan::PeripheralInfo {
        id: p.id(),
        local_name: p.local_name().unwrap_or_default(),
        details,
      }
    }).collect();

    let current = state.current_peripheral_id.as_ref().and_then(|id| {
      state.peripherals.iter().position(|p| p.id() == *id)
    }).unwrap_or_default();
    Self { data, current }
  }
}

pub trait PeripheralNameExt {
  fn local_name(&self) -> Option<String>;
  fn get_properties(&self) -> Option<PeripheralProperties>;
}

impl PeripheralNameExt for Peripheral {
  fn local_name(&self) -> Option<String> {
    self.get_properties().as_ref()?.local_name.clone()
  }

  fn get_properties(&self) -> Option<PeripheralProperties> {
    futures::executor::block_on(async { btleplug::api::Peripheral::properties(self).await }).ok()?
  }
}

impl State {
  pub fn new() -> Result<Self> {
    let adapters = futures::executor::block_on(async { Manager::new().await?.adapters().await })?;
    let current_adapter = adapters.get(0).cloned().unwrap();
    let peripherals = Vec::new();
    Ok(Self {
      adapters,
      current_adapter,
      peripherals,
      current_peripheral_id: None,
      peripheral_details: HashMap::new(),
    })
  }

  pub async fn update_peripherals(&mut self) -> Result<()> {
    Ok(self.peripherals = self.current_adapter.peripherals().await?)
  }
}

pub struct StateStore {
  state: State,
  last_err: Option<String>,
  props_tx: watch::Sender<Props>,
  action_rx: mpsc::Receiver<Action>,
  action_tx: mpsc::Sender<Action>,
}

impl StateStore {
  pub fn new() -> Result<(Self, watch::Receiver<Props>)> {
    let state = State::new()?;
    let (props_tx, props_rx) = watch::channel(Props::from(&state));
    let (action_tx, action_rx) = mpsc::channel(100);
    let store = Self {
      state,
      last_err: None,
      props_tx,
      action_rx,
      action_tx,
    };
    Ok((store, props_rx))
  }

  pub fn action_tx(&self) -> mpsc::Sender<Action> {
    self.action_tx.clone()
  }

  pub async fn run(mut self) -> Result<()> {
    self.state.current_adapter.start_scan(Default::default()).await?;
    loop {
      match self.action_rx.try_recv() {
        Ok(Action::Exit) => break Ok(()),
        Ok(action) => {
          let result = action.apply(&mut self.state).await;
          if let Err(ref e) = result {
            self.last_err = format!("{:?}: {:?}", action, e).into();
          }
        }
        _ => {},
      }
      self.state.update_peripherals().await.ok();
      self.props_tx.send((&self.state).into()).ok();
      tokio::time::sleep(std::time::Duration::from_micros(500)).await;
    }
  }
}
