#![no_std]
#![no_main]
extern crate alloc;

mod basic_result;

use crate::basic_result::{BasicError, BasicResult};
use alloc::format;
use cortex_m::prelude::*;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::mode::Blocking;
use embassy_stm32::usart::Uart;
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embassy_time::Delay;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[allow(clippy::unwrap_used)]
pub fn run() {
    let p = embassy_stm32::init(embassy_stm32::Config::default());
    let mut microcontroller_usart =
        Uart::new_blocking(p.USART1, p.PA10, p.PA9, usart::Config::default()).unwrap();
    microcontroller_usart.set_baudrate(1200).unwrap();
    let mut usb_usart =
        Uart::new_blocking(p.USART2, p.PA3, p.PA2, usart::Config::default()).unwrap();
    usb_usart.set_baudrate(115_200).unwrap();

    // Status LED
    let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    // Run the main logic. If something goes wrong, blink the status LED.
    if let Err(error) = main_logic(&mut microcontroller_usart, &mut usb_usart) {
        // Attempt to log the error message, but keep going if that fails.
        let _ = log(&mut usb_usart, format!("Error: {}", error.message).as_str());

        let mut delay = Delay;

        loop {
            led.toggle();
            delay.delay_ms(1000_u32);
        }
    }
}

fn main_logic(
    microcontroller_usart: &mut Uart<Blocking>,
    usb_usart: &mut Uart<Blocking>,
) -> BasicResult {
    log(usb_usart, "Hello world!")?;

    let mut buf = [0u8; 100];
    loop {
        microcontroller_usart
            .blocking_read(&mut buf)
            .map_err(BasicError::from_usart_error)?;
        log(usb_usart, "Got byte")?;
        usb_usart
            .blocking_write(&buf)
            .map_err(BasicError::from_usart_error)?;
    }
}

fn log(usb_usart: &mut Uart<Blocking>, message: &str) -> BasicResult {
    usb_usart
        .blocking_write(message.as_bytes())
        .map_err(|_| BasicError {
            message: "Log failed",
        })?;
    usb_usart
        .blocking_write("\r\n".as_ref())
        .map_err(|_| BasicError {
            message: "Log failed",
        })
}
