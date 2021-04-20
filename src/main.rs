#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_semihosting as _;
extern crate stm32f3xx_hal as hal;

use cortex_m::asm::{self, delay};
use cortex_m_rt::entry;
use hal::{pac, prelude::*};
use hal::serial::Serial;

use core::fmt::Write;

#[entry]
fn main() -> ! {
    // Get the peripherals
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();

    // Get the reset and control clock
    let mut rcc = dp.RCC.constrain();

    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze(&mut flash.acr);

    // Define the pins
    let tx_pin = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
    let rx_pin = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

    let serial = Serial::usart1(dp.USART1, (tx_pin, rx_pin), 9600.bps(), clocks, &mut rcc.apb2);

    let (mut tx, mut _rx) = serial.split();
    
    let dma1 = dp.DMA1.split(&mut rcc.ahb);

    let (mut tx_chan, mut rx_chan) = (dma1.ch4, dma1.ch5);

    let sending = tx.write_all("HELLO WORLD! IT WORKS!".as_bytes(), tx_chan);

    let (_, tx_chan, tx) = sending.wait();

    loop {
        asm::wfi();
    }
}
