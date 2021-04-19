#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_semihosting as _;

use cortex_m::{asm, singleton};
use cortex_m_rt::entry;
use stm32f3xx_hal::{pac, prelude::*, serial::Serial};


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let pins = (
        gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh),
        gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh),
    );
    let serial = Serial::usart1(dp.USART1, pins, 9600.bps(), clocks, &mut rcc.apb2);
    let (tx, rx) = serial.split();

    let dma1 = dp.DMA1.split(&mut rcc.ahb);

    let (tx_channel, rx_channel) = (dma1.ch4, dma1.ch5);

    // start separate DMAs for sending and receiving the data
    let mut tx_buf = b"hello there";
    let sending = tx.write_all(tx_buf, tx_channel);

    // Basically we reclaim the stuff here as its a blocking call
    let (tx_buf, tx_channel, tx) = sending.wait();

    loop {
        asm::wfi();
    }
}
// fn main() -> ! {
//     let dp = pac::Peripherals::take().unwrap();

//     let mut flash = dp.FLASH.constrain();
//     let mut rcc = dp.RCC.constrain();

//     let clocks = rcc
//         .cfgr
//         .use_hse(8u32.mhz())
//         .sysclk(48u32.mhz())
//         .pclk1(24u32.mhz())
//         .pclk2(24u32.mhz())
//         .freeze(&mut flash.acr);

//     assert!(clocks.usbclk_valid());

//     let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
//     let mut led = gpioe
//         .pe13
//         .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

//     led.set_low().ok();

//     let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

//     let mut usb_dp = gpioa
//         .pa12
//         .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
//     usb_dp.set_low().ok();
//     delay(clocks.sysclk().0 / 100);

//     let usb_dm = gpioa.pa11.into_af14(&mut gpioa.moder, &mut gpioa.afrh);
//     let usb_dp = usb_dp.into_af14(&mut gpioa.moder, &mut gpioa.afrh);

//     let usb = Peripheral {
//         usb: dp.USB,
//         pin_dm: usb_dm,
//         pin_dp: usb_dp,
//     };

//     let usb_bus = UsbBus::new(usb);

//     let mut serial = SerialPort::new(&usb_bus);

//     let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
//         .product("Roc 1")
//         .device_class(USB_CLASS_CDC)
//         .build();

//     // Query the gyroscope
//     // Query the E-compass
//     loop {
//         if !usb_device.poll(&mut [&mut serial]) {
//             continue;
//         }

//         let mut buf = [0u8; 64];

//         match serial.read(&mut buf) {
//             Ok(count) if count > 0 => {
//                 led.set_high().ok(); // Turn on

//                 // Echo back in upper case
//                 for c in buf[0..count].iter_mut() {
//                     if 0x61 <= *c && *c <= 0x7a {
//                         *c &= !0x20;
//                     }
//                 }

//                 let mut write_offset = 0;
//                 while write_offset < count {
//                     match serial.write(&buf[write_offset..count]) {
//                         Ok(len) if len > 0 => {
//                             write_offset += len;
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//             _ => {}
//         }

//         led.set_low().ok(); // Turn off
//     }
// }
