# Day 2 - The onboard sensors

The goal of today is to get the onboard sensors working and sending the data over serial

[[toc]]

## L3GD20

Okay, I spent 3 days trying to get this to work, first learning SPI, which is a rather simple and nice protocol, going through the datasheets of my board, the 2 potential onboard sensors and drivers written for the sensor. I kept retrieving garbage values from the bus, until... I realized I had mistyped the chip select pin, yea that pin that activated the SPI communication, wrong.

![AHh](/spongebob_dispare.jpg)

### How to do it

* Write about how to implement it

## LSM303AGR

This e-compass sensor combines a magnetometer and accelerometer together and allows you to access them either through SPI or I2C, however the board I have only allows access through I2C. After taking some time to understand the datasheet, which I am getting a **lot** better at now, and the I2C protocol, the implementation went fairly smoothly. Although I was stumped for a day and a bit until I realized I was trying to address registers rather than the actual slave address. One thing I would like to do a bit later is dive into the datasheet of the MCU again and really understand how to do it from scratch as I feel reliant on the HAL at the moment.

### How to do it

* Write about implementation or just show it

## Hooking it up with the groundstation

Since I now had the ability to read the sensors and write to serial port on my laptop I figured I could make the groundstation demo!

<iframe width="560" height="315" src="https://www.youtube.com/embed/_YyYEwULC4Q" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

Apologizes for the poor quality in the video but hopefully you can see how some of the data collecting is working!
Also for anyone who looks at the current code, please do not think this is what I consider production ready code. I am simply prototyping and will rewrite it when I feel I have a good design in mind.

## Conclusion

This part of the project took me 6 days to complete, of which I spent about an hour and a half on the project each day. I am pretty happy with the results, not cramming it into one day makes it far less stressfull than other projects and it's really nice to feel that I am not working in someone else's framework.

## Further work

* Put the sensors in the correct units
* Figure out the best configurations for each of the sensors
* Write better wrappers for the sensors
* Figure out how the magnetometer works to get orientation

## Scratch notes

<iframe src="/day2.pdf" width="100%" height="800px">
</iframe>