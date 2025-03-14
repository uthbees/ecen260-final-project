// Set up the entrypoint and the panic handlers. Business logic should go in other files.

#![no_main]
#![no_std]

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    cortex_m4_binary::run();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
