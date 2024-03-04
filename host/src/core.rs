use btleplug::api::{BDAddr, PeripheralProperties};
use btleplug::api::{bleuuid::uuid_from_u16, Central, Characteristic, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral};
use anyhow::Result;

trait PeripheralNameExt {
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
// Replace with the actual UUID of the characteristic you want to read
const CHARACTERISTIC_UUID: u16 = 0x2a23;

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
