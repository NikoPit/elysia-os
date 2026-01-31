#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::testing::run_tests)]

pub mod debug_exit;
pub mod interrupts;
pub mod os;
pub mod panic_handler;
pub mod serial_print;
pub mod testing;
pub mod vga_print;

#[cfg(test)]
use core::panic::PanicInfo;

use crate::os::get_os;

use x86_64::instructions::interrupts::int3;

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use crate::panic_handler::test_handle_panic;

    test_handle_panic(_info);
}
