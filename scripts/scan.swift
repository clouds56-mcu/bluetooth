import CoreBluetooth
import Darwin

// Define a service UUID to filter results (optional)
let serviceUUID: [CBUUID]? = nil // nil for all services

// CBCentralManagerDelegate protocol for handling scan events
class MyCentralManagerDelegate: NSObject, CBCentralManagerDelegate {
  func centralManagerDidUpdateState(_ central: CBCentralManager) {
    if central.state == .poweredOn {
      print("Central manager is powered on, starting scan...")
      central.scanForPeripherals(withServices: serviceUUID, options: nil)
    } else {
      print("Central manager state: \(central.state)")
    }
  }

  func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String : Any], rssi RSSI: NSNumber) {
    print("Discovered peripheral:")
    print("  Name: \(peripheral.name ?? "Unknown")")
    print("  Identifier: \(peripheral.identifier)")
    print("  RSSI: \(RSSI)")
    print("  Advertisement data: \(advertisementData)")
    print("-----")
  }
}

// Create central manager instance and set delegate
let delegate = MyCentralManagerDelegate()
let centralManager = CBCentralManager(delegate: delegate, queue: nil)

print("Scanning for BLE devices...")

// Run the main run loop to keep the program running
signal(SIGINT) { _ in exit(0) }
RunLoop.main.run()
