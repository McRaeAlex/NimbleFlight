# Overview 

This drone project is my attempt at really learning embedded programming.

Each "day" I work on the project I will create a blog entry about it. A day in this context is basically just a group of tasks which I initially think I can complete in a day but end up completing in multiple. 👌

**It should be noted, this blog is not meant for anyones eyes but my own! It's not meant to be a good read, or informative, just keep me motivated to keep going. The documentation section will be the place for information**

## Blog Entries

[day0 - Inital groundstation setup and design](./day0)\
[day1 - Outputing data over the USB](./day1)\
[day2 - Getting the sensor data and hooking it up to the groundstation](./day2)\
[day3 - Recieving data from the handheld controller](./day3)

## The stack

For the drone flight controller I am using a stm32f3 discovery board, which the rust [discovery book](https://docs.rust-embedded.org/discovery/) is based on.

I will be using the built in gyroscope, accelerometer, and other on board sensors.

For the electronic speed controller I will be using **idk** yet.

For the motors I will be using **idk** yet.

The groundstation is written in javascript (node on the backend, vanilla on the frontend)
