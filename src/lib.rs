#![no_std]
#![no_main]

use cortex_m::prelude::*;
use embassy_stm32::Config;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Delay;

pub fn run() {
    let per = embassy_stm32::init(Config::default());
    let mut delay = Delay;

    let mut led = Output::new(per.PA5, Level::High, Speed::Low);

    loop {
        delay.delay_ms(1000_u32);
        led.toggle();
    }
}
