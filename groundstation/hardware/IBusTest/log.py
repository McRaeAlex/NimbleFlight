# Use the .ino file in the same dir on a arduino mega 2560
# Install pyserial to run this
import serial

with serial.Serial('/dev/ttyACM0', 115200, timeout=1) as ser:
    print(ser.name)
    with open('./ibus_serial_out.hex', 'wb') as f:
        f.truncate(0)
        f.seek(0)

        while(True):
            packet = ser.read(32)
            print(packet)
            f.write(packet)


