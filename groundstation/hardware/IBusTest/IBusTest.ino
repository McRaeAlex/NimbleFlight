
#define POLL_RATE 115200

void setup() {
  // put your setup code here, to run once:
  Serial.begin(POLL_RATE);
  Serial1.begin(POLL_RATE);
}

void loop() {
  // put your main code here, to run repeatedly:
  if (Serial1.available() > 0) {
    Serial.write(Serial1.read());
  }
}
