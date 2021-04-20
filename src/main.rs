#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_semihosting as _;
extern crate stm32f3xx_hal as hal;

mod l3gd20;

use cortex_m::asm::{self, delay};
use cortex_m_rt::entry;
use hal::{
    gpio::{Output, PushPull},
    pac::SPI1,
    serial::Serial,
};
use hal::{pac, prelude::*};
use hal::{
    pac::usart1::isr::REACK_R,
    spi::{Mode, Phase, Polarity, Spi},
};
use l3gd20::Registers;

use core::{convert::Infallible, fmt::Write};

#[entry]
fn main() -> ! {
    // Get the peripherals
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();

    // Get the reset and control clock
    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

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

    let (mut tx, mut _rx) = serial.split();

    let dma1 = dp.DMA1.split(&mut rcc.ahb);

    let (mut tx_chan, mut rx_chan) = (dma1.ch4, dma1.ch5);

    let sending = tx.write_all("HELLO WORLD! IT WORKS!".as_bytes(), tx_chan);

    let (_, mut tx_chan, mut tx) = sending.wait();

    // Onboard sensor init
    // -------------------
    let mut cs = gpioa
        .pa4
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let sclk = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    let mut spi = Spi::spi1(
        dp.SPI1,
        (sclk, miso, mosi),
        spi_mode,
        10.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    // TODO: probably want to setup the values we send
    let mut msg_sending = [
        Registers::CTRL_REG1 as u8,
        0x0f, // turn on the gyroscope and enable all 3 axis and set data rate
        Registers::CTRL_REG4 as u8,
        0b1000_0000, // Turn on the blockdata update, we set the byte ording to little endian,
    ];
    let _msg_recieved = spi.transfer(&mut msg_sending).unwrap();

    let buf = "gyro_ret".as_bytes();
    loop {
        let val = read_accel_values(&mut spi, &mut cs).unwrap();

        let (_, tx_chan2, tx2) = tx.write_all(buf, tx_chan).wait();
        tx = tx2;
        tx_chan = tx_chan2;
    }
}

use hal::gpio::gpioa::{PA4, PA5, PA6, PA7};
use hal::gpio::AF5;
fn read_accel_values(
    spi: &mut Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>), u8>,
    cs: &mut PA4<Output<PushPull>>,
) -> Result<(i16, i16, i16), Infallible> {
    cs.set_low().ok();
    // Check if new data is avaliable
    loop {
        let mut buf = [Registers::STATUS_REG as u8, 0];
        let reply = spi.transfer(&mut buf).unwrap();

        if reply[1] & 0b0000_0100 != 0 {
            // This means there is new data
            break;
        }

        // if we check the 7th bit of the status register we can check the data vs read rate to see if we are missing values
    }

    let mut commands = [
        // This should read out the values
        Registers::OUT_X_L as u8,
        Registers::OUT_X_H as u8,
        Registers::OUT_Y_L as u8,
        Registers::OUT_Y_H as u8,
        Registers::OUT_Z_L as u8,
        Registers::OUT_Z_H as u8,
        0x00,
    ];

    let reply = spi.transfer(&mut commands).unwrap();

    cs.set_high().ok();

    assert!(reply.len() == 7);

    let x: i16 = i16::from_le_bytes([reply[1], reply[2]]);
    let y: i16 = i16::from_le_bytes([reply[3], reply[4]]);
    let z: i16 = i16::from_le_bytes([reply[5], reply[6]]);

    Ok((x, y, z))
}
