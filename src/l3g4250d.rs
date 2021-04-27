use hal::gpio::PushPull;
use hal::gpio::{Output, AF5};
use hal::pac::SPI1;
use hal::spi::Spi;
use hal::gpio::gpioa::{PA5, PA6, PA7};
use hal::gpio::gpioe::{PE3};
use cortex_m::prelude::*;
use hal::prelude::_embedded_hal_digital_OutputPin;

const READ_BIT: u8 = 1 << 7;
const WRITE_BIT: u8 = !(1 << 7); // Set all bits so we can and it
const MULTI_BIT: u8 = 1 << 6;
const SINGLE_BIT: u8 = !(1 << 6); // Set all other bits so we can and with it

fn read(reg: u8) -> u8 {
    reg | READ_BIT
}

fn write(reg: u8) -> u8 {
    reg & WRITE_BIT
}

fn multi(reg: u8) -> u8 {
    reg | MULTI_BIT
}

fn single(reg: u8) -> u8 {
    reg & SINGLE_BIT
}

pub const MODE: hal::spi::Mode = hal::spi::Mode {
   polarity: hal::spi::Polarity::IdleHigh,
   phase: hal::spi::Phase::CaptureOnSecondTransition,
};

#[allow(non_camel_case_types, dead_code)]
#[repr(u8)]
pub enum Registers {
    WHO_AM_I = 0x0F,
    CTRL_REG1 = 0x20,
    CTRL_REG2 = 0x21,
    CTRL_REG3 = 0x22,
    CTRL_REG4 = 0x23,
    CTRL_REG5 = 0x24,
    REFERENCE = 0x25,
    OUT_TEMP = 0x26,
    STATUS_REG = 0x27,
    OUT_X_L = 0x28,
    OUT_X_H = 0x29,
    OUT_Y_L = 0x2A,
    OUT_Y_H = 0x2B,
    OUT_Z_L = 0x2C,
    OUT_Z_H = 0x2D,
    FIFO_CTRL_REG = 0x2E,
    FIFO_SRC_REG = 0x2F,
    INT1_CFG = 0x30,
    INT1_SRC = 0x31,
    INT1_THS_XH = 0x32,
    INT1_THS_XL = 0x33,
    INT1_THS_YH = 0x34,
    INT1_THS_YL = 0x35,
    INT1_THS_ZH = 0x36,
    INT1_THS_ZL = 0x37,
    INT1_DURATION = 0x38,
}

pub struct I3g4250d {
    pub spi: Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>), u8>,
    pub cs: PE3<Output<PushPull>>,
}

impl I3g4250d {
    pub fn who_am_i(&mut self) -> u8 {
        self.register_read(Registers::WHO_AM_I)
    }

    pub fn register_read(&mut self, reg: Registers) -> u8 {
        self.cs.set_low().ok();

        let mut buffer = [read(single(reg as u8)), 0x00];
        let data = self.spi.transfer(&mut buffer).unwrap();

        self.cs.set_high().ok();

        assert!(data.len() == 2);

        data[1]
    }

    pub fn register_write(&mut self, reg: Registers, value: u8) {
        self.cs.set_low().ok();

        let mut buffer = [write(single(reg as u8)), value];
        let _data = self.spi.transfer(&mut buffer).unwrap();

        self.cs.set_high().ok();
    }

    pub fn register_read_many<'w>(&mut self, buf: &'w mut [u8]) -> &'w [u8] {
        assert!(buf.len() > 1);

        buf[0] = read(multi(buf[0].into()));
        self.cs.set_low().ok();

        let data = self.spi.transfer(buf).unwrap();

        self.cs.set_high().ok();

        data
    }

    // pub fn register_write_many<'w>(&mut self, buf: &'w mut [u8]) {
    //     assert!(buf.len() > 1);

    //     buf[0] = read(multi(buf[0].into()));
    //     self.cs.set_low().ok();

    //     let data = self.spi.transfer(buf).unwrap();

    //     self.cs.set_high().ok();
    // }


    pub fn values(&mut self) -> (i16, i16, i16) {
        let mut buf = [Registers::OUT_X_L as u8, 0, 0, 0, 0, 0, 0];

        let result = self.register_read_many(&mut buf);
        assert!(result.len() == 7);

        let x = i16::from_le_bytes([result[1], result[2]]);
        let y = i16::from_le_bytes([result[3], result[4]]);
        let z = i16::from_le_bytes([result[5], result[6]]);

        (x, y, z)
    }
}
