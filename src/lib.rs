#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::testing::run_tests)]

pub mod debug_exit;
pub mod driver;
pub mod gdt;
pub mod hardware_interrupt;
pub mod interrupts;
pub mod keyboard;
pub mod misc;
pub mod os;
pub mod panic_handler;
pub mod serial_print;
pub mod testing;
pub mod tss;
pub mod vga_print;

#[cfg(test)]
use core::panic::PanicInfo;

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use crate::{misc::hlt_loop, os::get_os};

    get_os().init();

    test_main();

    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use crate::panic_handler::test_handle_panic;

    test_handle_panic(_info);
}
