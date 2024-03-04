void setup() {
  delay(1000);
  Serial.begin(9600);
  Serial.println("hello world?");
  while(!Serial);
  if (Serial) {
    Serial.println("hello world!");
  }
}

void loop() {
  String s = Serial.readString();
  if (!s.isEmpty()) {
    Serial.println(s);
  }
}
