extern crate stm32f3xx_hal as hal;


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

