# The onboard sensors

The goal of today is to get the onboard sensors working and sending the data over serial

## L3GD20

Okay, I spent 3 days trying to get this to work, first learning SPI, which is a rather simple and nice protocol, going through the datasheets of my board, the 2 potential onboard sensors and drivers written for the sensor. I kept retrieving garbage values from the bus, until... I realized I had mistyped the chip select pin, yea that pin that activated the SPI communication, wrong.

![AHh](./resource/spongebob_how.webp)

### How to do it

* Write about how to implement it

##