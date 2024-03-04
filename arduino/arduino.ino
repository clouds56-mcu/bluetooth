#include <Arduino.h>
#include <ArduinoBLE.h>

BLEService batteryService("1101");
BLEStringCharacteristic batteryLevelChar("2101", BLERead | BLENotify, 0);

void setup() {
  delay(1000);
  Serial.begin(9600);
  Serial.println("hello world?");
  while(!Serial);
  if (!BLE.begin()) {
    Serial.println("starting BLE failed!");
    while (1);
  }
  BLE.setLocalName("BatteryMonitor");
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

    if (!central.connected()) {
      central.connect();
    }
  }
  delay(1000);
}
