use cortex_m::prelude::*;
use hal::{
    gpio::{
        gpiob::{PB6, PB7},
        AF4,
    },
    i2c::I2c,
    pac::I2C1,
};

const ACCELEROMETER_ADDR: u8 = 0x19;
const MAGNETOMETER_ADDR: u8 = 0x1E;

pub struct Lsm303agr {
    bus: I2c<I2C1, (PB6<AF4>, PB7<AF4>)>,
}

impl Lsm303agr {
    pub fn new(bus: I2c<I2C1, (PB6<AF4>, PB7<AF4>)>) -> Self {
        Self { bus }
    }

    pub fn who_am_i(&mut self) -> (u8, u8) {
        let mut buffer = [0];
        self.bus
            .write_read(
                ACCELEROMETER_ADDR,
                &[Registers::WHO_AM_I_A as u8],
                &mut buffer,
            )
            .unwrap();
        let mut buffer2 = [0];
        self.bus
            .write_read(
                MAGNETOMETER_ADDR,
                &[Registers::WHO_AM_I_M as u8],
                &mut buffer2,
            )
            .unwrap();
        (buffer[0], buffer2[0])
    }

    pub fn turn_on(&mut self) {
        self.bus
            .write(
                ACCELEROMETER_ADDR,
                &[Registers::CTRL_REG1_A as u8, 0b0100_0_111],
            )
            .unwrap(); // This turns it on // This turns it on
        self.bus
            .write(
                MAGNETOMETER_ADDR,
                &[Registers::CFG_REG_A_M as u8, 0b0_0_0_0_10_00],
            )
            .unwrap(); // Turns on the device and makes it poll at 50hz
    }

    pub fn magnetometer_values(&mut self) -> (i16, i16, i16) {
        let mut buffer = [0; 6];
        let register = Registers::OUTX_L_REG_M as u8 | 0b1000_0000;
        self.bus
            .write_read(MAGNETOMETER_ADDR, &[register], &mut buffer)
            .unwrap();

        let mx = i16::from_le_bytes([buffer[0], buffer[1]]);
        let my = i16::from_le_bytes([buffer[2], buffer[3]]);
        let mz = i16::from_le_bytes([buffer[4], buffer[5]]);
        (mx, my, mz)
    }

    pub fn values(&mut self) -> (i16, i16, i16) {
        let mut buffer_a = [0; 6];
        let register_a = (Registers::OUT_X_L_A as u8) | 0b1000_0000; // this makes it a read command
        self.bus
            .write_read(ACCELEROMETER_ADDR, &[register_a], &mut buffer_a)
            .unwrap();

        let ax = i16::from_le_bytes([buffer_a[0], buffer_a[1]]);
        let ay = i16::from_le_bytes([buffer_a[2], buffer_a[3]]);
        let az = i16::from_le_bytes([buffer_a[4], buffer_a[5]]);
        (ax, ay, az)
    }
}

#[allow(non_camel_case_types, dead_code)]
#[repr(u8)]
pub enum Registers {
    STATUS_REG_AUX_A = 0x07,
    OUT_TEMP_L_A = 0x0C,
    OUT_TEMP_H_A = 0x0D,
    INT_COUNTER_REG_A = 0x0E,
    WHO_AM_I_A = 0x0F,
    TEMP_CFG_REG_A = 0x1F,
    // Accelerometer control registers
    CTRL_REG1_A = 0x20,
    CTRL_REG2_A = 0x21,
    CTRL_REG3_A = 0x22,
    CTRL_REG4_A = 0x23,
    CTRL_REG5_A = 0x24,
    CTRL_REG6_a = 0x25,
    REFERENCE_DATACAPTURE_A = 0x26,
    STATUS_REG_A = 0x27,
    // Accelerometer output registers
    OUT_X_L_A = 0x28,
    OUT_X_H_A = 0x29,
    OUT_Y_L_A = 0x2A,
    OUT_Y_H_A = 0x2B,
    OUT_Z_L_A = 0x2C,
    OUT_Z_H_A = 0x2D,
    // FIFO Registers
    FIFO_CTRL_REG_A = 0x2E,
    FIFO_SRC_REG_A = 0x2F,
    // Interrupt 1 registers
    INT1_CFG_A = 0x30,
    INT1_SRC_A = 0x31,
    INT1_THS_A = 0x32,
    INT1_DURATION_A = 0x33,
    // Interrupt 2 registers
    INT2_CFG_A = 0x34,
    INT2_SRC_A = 0x35,
    INT2_THS_A = 0x36,
    INT2_DURATION_A = 0x37,

    CLICK_CFG_A = 0x38,
    CLICK_SRC_A = 0x39,
    CLICK_THS_A = 0x3A,

    TIME_LIMIT_A = 0x3B,
    TIME_LATENCY_A = 0x3C,
    TIME_WINDOW_A = 0x3D,

    Act_THS_A = 0x3E,
    Act_DUR_A = 0x3F,

    OFFSET_X_REG_L_M = 0x45,
    OFFSET_X_REG_H_M = 0x46,
    OFFSET_Y_REG_L_M = 0x47,
    OFFSET_Y_REG_H_M = 0x48,
    OFFSET_Z_REG_L_M = 0x49,
    OFFSET_Z_REG_H_M = 0x4A,

    WHO_AM_I_M = 0x4F,

    CFG_REG_A_M = 0x60,
    CFG_REG_B_M = 0x61,
    CFG_REG_C_M = 0x62,

    INT_CTRL_REG_M = 0x63,
    INT_SOURCE_REG_M = 0x64,
    INT_THS_L_REG_M = 0x65,
    INT_THS_H_REG_M = 0x66,

    STATUS_REG_M = 0x67,

    OUTX_L_REG_M = 0x68,
    OUTX_H_REG_M = 0x69,
    OUTY_L_REG_M = 0x6A,
    OUTY_H_REG_M = 0x6B,
    OUTZ_L_REG_M = 0x6C,
    OUTZ_H_REG_M = 0x6D,
}
