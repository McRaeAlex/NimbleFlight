use core::fmt::Write;
use hal::nb;
use hal::serial::Tx;
use hal::{pac::USART1, prelude::_embedded_hal_serial_Write};

pub struct Logger {
    tx: Tx<USART1>,
}

// TODO: verify that these are true. Though we are single threaded... but Rust doesn't know that
// Since we are single threaded this is fine.. However we should come up with a new solution later
unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

pub struct Values {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub ax: i16,
    pub ay: i16,
    pub az: i16,
}

impl Logger {
    pub fn init(tx: Tx<USART1>) -> Self {
        let logger = Logger { tx: tx };
        logger
    }

    pub fn log(&mut self, record: Values) {
        writeln!(
            self,
            "{} {} {} {} {} {}",
            record.x, record.y, record.z, record.ax, record.ay, record.az
        ).ok();
    }
}

// impl log::Log for core::cell::RefCell<Logger> {
//     fn enabled(&self, metadata: &Metadata) -> bool {
//         true
//     }

//     fn log(&self, record: &Record<'_>) {
//         writeln!(self, "{}:{} -- {}", record.level(), record.target(), record.args());
//         // TODO: depending on the log level do different things
//     }
//     fn flush(&self) {}
// }

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.as_bytes() {
            nb::block!(self.tx.write(*ch)).unwrap();
        }
        Ok(())
    }
}
