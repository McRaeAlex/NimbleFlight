# GPIO

## Enable and setup

1. Enable the PORT with the peripheral clock enable register which is in the rcc section of the documentation. You will have to lookup the port in the table to figure out which register controls it. Generally setting the bits to a 1 enabled it. **modify the register don't just write to it.**
2. Set the direction of the pin, this is in a mode register (MODER).
3. Use it