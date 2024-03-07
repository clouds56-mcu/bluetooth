#include <Arduino.h>
#include <ArduinoBLE.h>

BLEService batteryService("1101");
BLEStringCharacteristic batteryLevelChar("2101", BLERead | BLENotify, 0);

void setup() {
  delay(1000);
  Serial.begin(115200);
  Serial.println("hello world?");
  while(!Serial);
  if (!BLE.begin()) {
    Serial.println("starting BLE failed!");
    while (1);
  }
  BLE.setLocalName("BatteryMonitor");
  BLE.setDeviceName("BatteryMonitor");
  // q: what is device name and local name?
  // a: device name is the name that shows up in the bluetooth settings
  //    local name is the name that shows up in the advertising packet
  //    if local name is not set, device name is used
  //    if device name is not set, local name is used
  //    if neither are set, the device address is used
  //    if the device address is not set, the device name is set to "Arduino"
  //    if the device address is set, the device name is set to "Arduino-<address>"
  BLE.setAdvertisedService(batteryService);
  batteryService.addCharacteristic(batteryLevelChar);
  BLE.addService(batteryService);

  BLE.advertise();
  if (Serial) {
    Serial.println("hello world!");
  }
}

void loop() {
  String s = Serial.readString();
  if (!s.isEmpty()) {
    Serial.println(s);
  }

  BLEDevice central = BLE.central();
  if (central) {
    Serial.print("Connected to central: ");
    Serial.println(central.address());
    Serial.print(1);

    if (!central.connected()) {
      central.connect();
    }
  }
  delay(1000);
}
