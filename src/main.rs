#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_semihosting as _;
extern crate stm32f3xx_hal as hal;

mod l3gd20;

use arrayvec::ArrayString;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use hal::serial::Serial;
use hal::spi::Spi;
use hal::{pac, prelude::*};
use l3gd20::Registers;

use core::{cell::RefCell, fmt::Write};

const LOG_SIZE: usize = 255;

#[entry]
fn main() -> ! {
    let mut log_buf = [0; LOG_SIZE];
    // Get the peripherals
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();

    // Get the reset and control clock
    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    // UART INIT
    // ---------
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);

    // Define the pins
    let tx_pin = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
    let rx_pin = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

    let serial = Serial::usart1(
        dp.USART1,
        (tx_pin, rx_pin),
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );

    let (tx, _rx) = serial.split();

    let dma1 = dp.DMA1.split(&mut rcc.ahb);

    let (tx_chan, _rx_chan) = (dma1.ch4, dma1.ch5);

    let sending = tx.write_all("HELLO WORLD! IT WORKS!".as_bytes(), tx_chan);

    let (_, mut tx_chan, mut tx) = sending.wait();

    // Onboard sensor init
    // -------------------
    let mut cs = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    cs.set_high().ok();
    let sclk = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    // Set up the gyro
    let mut gyro = l3gd20::L3gd20 {
        spi: Spi::spi1(
            dp.SPI1,
            (sclk, miso, mosi),
            l3gd20::MODE,
            1.mhz(),
            clocks,
            &mut rcc.apb2,
        ),
        cs: cs,
    };

    gyro.register_write(Registers::CTRL_REG1, 0b00_00_1_111); // Turns it on

    let _a = gyro.who_am_i();

    loop {
        let (x, y, z) = gyro.values();
        {
        update_log_buf(&mut log_buf, x, y, z);
        }
        // Extremely important: MUST not change this to be non_blocking
        {
        let (_, tx_chan2, tx2) = tx.write_all( &mut log_buf, tx_chan).wait();

        delay(1000);
        tx = tx2;
        tx_chan = tx_chan2; }
    }
}

fn update_log_buf(log_buf: &mut [u8; LOG_SIZE], x: i16, y: i16, z: i16) {
    let mut log = ArrayString::<LOG_SIZE>::new(); // This is to get the core::fmt::Write trait
    write!(&mut log, "gyro: {{ x: {}, y: {}, z: {} }}\n", x, y, z).expect("Failed to format");

    log_buf[..log.len()].copy_from_slice(log.as_bytes()); // They must be the same size so we do this
}
