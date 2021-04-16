

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  Serial1.begin(9600);
//  pinMode(13, OUTPUT);
}

void loop() {
  // put your main code here, to run repeatedly:
  if (Serial1.available() > 0) {
    Serial.write(Serial1.read());
  }
//  delay(500);
//  digitalWrite(13, HIGH);
//  delay(500);
//  digitalWrite(13, LOW);
}
