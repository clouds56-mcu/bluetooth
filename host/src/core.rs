use btleplug::api::PeripheralProperties;
use btleplug::api::{Central as _, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral};
use anyhow::Result;
use tokio::sync::watch::{Receiver, Sender};

use crate::ui::{self, UiState};

pub struct State {
  pub adapters: Vec<Adapter>,
  pub current_adapter: Adapter,
  pub peripherals: Vec<Peripheral>,
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
        local_name: p.local_name().unwrap_or_default(),
        details,
      }
    }).collect();
    Self {
      data,
      current: 0,
    }
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
    let peripherals = futures::executor::block_on(async { current_adapter.peripherals().await })?;
    Ok(Self {
      adapters,
      current_adapter,
      peripherals,
    })
  }

  pub async fn update_peripherals(&mut self) -> Result<()> {
    Ok(self.peripherals = self.current_adapter.peripherals().await?)
  }
}

pub struct StateStore {
  state: State,
  state_tx: Sender<UiState>,
}

impl StateStore {
  pub fn new() -> Result<(Self, Receiver<UiState>)> {
    let state = State::new()?;
    let (state_tx, state_rx) = tokio::sync::watch::channel(UiState::from(&state));
    Ok((Self { state, state_tx }, state_rx))
  }

  pub async fn run(mut self) -> Result<()> {
    self.state.current_adapter.start_scan(Default::default()).await?;
    loop {
      self.state.update_peripherals().await.ok();
      self.state_tx.send((&self.state).into()).ok();
      tokio::time::sleep(std::time::Duration::from_micros(500)).await;
    }
  }
}

#[tokio::main]
pub async fn demo() -> Result<()> {
  // Get the first Bluetooth adapter
  let manager = Manager::new().await?;
  let adapters = manager.adapters().await?;
  let central = adapters.into_iter().nth(0).unwrap();//.central().await?;

  // Start scanning for BLE devices
  central.start_scan(Default::default()).await?;

  // Wait for 2 seconds for devices to be discovered
  tokio::time::sleep(std::time::Duration::from_secs(5)).await;

  // Find the device with a specific name (replace with your device name)
  let device = central
    .peripherals()
    .await?
    .into_iter()
    .find(|peripheral| {
      println!("found {} {:?}", peripheral.local_name().unwrap_or_default(), peripheral.get_properties());
      peripheral.local_name().unwrap_or_default() == "Arduino"
    })
    .unwrap();

  // Connect to the device
  device.connect().await?;

  // Discover services and characteristics
  device.discover_services().await?;

  // Find the characteristic by UUID
  let characteristic = device
    .characteristics()
    .into_iter()
    .for_each(|c| println!("char: {}, {:?}", c.uuid, c));

  // Read the value of the characteristic

  // Print the value (may need conversion based on characteristic format)
  println!("Characteristic value: {:?}", characteristic);

  Ok(())
}
