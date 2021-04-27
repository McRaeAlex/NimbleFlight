#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_semihosting as _;
extern crate stm32f3xx_hal as hal;
// #[macro_use] extern crate log;

mod l3g4250d;
mod lsm303agr;
mod logger;

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use hal::i2c::I2c;
use hal::serial::Serial;
use hal::spi::Spi;
use hal::{pac, prelude::*};
use l3g4250d::Registers;
use logger::Logger;
use lsm303agr::Lsm303agr;

#[entry]
fn main() -> ! {
    // Get the peripherals
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();

    // Get the reset and control clock
    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
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

    let mut logger = Logger::init(tx);

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
    let mut gyro = l3g4250d::I3g4250d {
        spi: Spi::spi1(
            dp.SPI1,
            (sclk, miso, mosi),
            l3g4250d::MODE,
            1.mhz(),
            clocks,
            &mut rcc.apb2,
        ),
        cs: cs,
    };

    let scl = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::new(dp.I2C1, (sda, scl), 50.khz(), clocks, &mut rcc.apb1);
    let mut compass = Lsm303agr::new(i2c);
    compass.turn_on();

    gyro.register_write(Registers::CTRL_REG1, 0b00_00_1_111); // Turns it on

    let _a = gyro.who_am_i();
    let _b = compass.who_am_i();

    loop {
        let (x, y, z) = gyro.values();
        let (ax, ay, az) = compass.values();

        logger.log(logger::Values { x: x, y: y, z: z, ax, ay, az });

        delay(1000);
    }
}
