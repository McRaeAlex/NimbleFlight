# Hermes

This drone project is my attempt at really learning embedded programming.

Each "day" I work on the project I will create a blog entry about it. A day in this context is basically just a group of tasks which I initially think I can complete in a day but end up completing in multiple. 👌

## Blog Entries

[day0 - Inital groundstation setup and design](./day0)

## The stack

For the drone flight controller I am using a stm32f3 discovery board, which the rust [discovery book](https://docs.rust-embedded.org/discovery/) is based on.

I will be using the built in gyroscope, accelerometer, and other on board sensors.

For the electronic speed controller I will be using **idk** yet.

For the motors I will be using **idk** yet.

The groundstation is written in javascript (node on the backend, vanilla on the frontend)
