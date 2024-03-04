use btleplug::api::{bleuuid::uuid_from_u16, Central, Characteristic, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use tokio::time::sleep;

trait PeripheralNameExt {
  fn local_name(&self) -> Option<String>;
}

impl PeripheralNameExt for Peripheral {
  fn local_name(&self) -> Option<String> {
    let properties = futures::executor::block_on(async { self.properties().await }).ok()?;
    properties.as_ref()?.local_name.clone()
  }
}

// Replace with the actual UUID of the characteristic you want to read
const CHARACTERISTIC_UUID: u16 = 0x2a23;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // Get the first Bluetooth adapter
  let manager = Manager::new().await?;
  let adapters = manager.adapters().await?;
  let central = adapters.into_iter().nth(0).unwrap();//.central().await?;

  // Start scanning for BLE devices
  central.start_scan(Default::default()).await?;

  // Wait for 2 seconds for devices to be discovered
  sleep(std::time::Duration::from_secs(2)).await;

  // Find the device with a specific name (replace with your device name)
  let device = central
    .peripherals()
    .await?
    .into_iter()
    .find(|peripheral| {
      println!("found {}", peripheral.local_name().unwrap_or_default());
      peripheral.local_name().unwrap_or_default() == "Your Device Name"
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
    .find(|c| c.uuid == uuid_from_u16(CHARACTERISTIC_UUID))
    .unwrap();

  // Read the value of the characteristic

  // Print the value (may need conversion based on characteristic format)
  println!("Characteristic value: {:?}", characteristic);

  Ok(())
}
