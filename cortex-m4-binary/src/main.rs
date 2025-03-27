// Set up the global stuff, like the entrypoint and panic handler. Business logic should go in other files.

#![no_main]
#![no_std]

extern crate alloc;

use cortex_m::prelude::*;
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use cortex_m_rt::entry;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Delay;

#[entry]
fn main() -> ! {
    // Initialize the allocator - see https://docs.rs/embedded-alloc/latest/embedded_alloc/
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    cortex_m4_binary::run();

    #[allow(clippy::empty_loop)]
    loop {}
}

/// Blink the status LED quickly if we panic
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let p = unsafe { embassy_stm32::Peripherals::steal() };
    let mut led = Output::new(p.PA5, Level::High, Speed::Low);
    let mut delay = Delay;

    loop {
        led.toggle();
        delay.delay_ms(100_u32);
    }
}
