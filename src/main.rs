// Disable dynamic linking with the std library because there is no std library in our own os
#![no_std]
// Disables main function to customize entry point
#![no_main]
#![feature(custom_test_frameworks)]
// renames main function for testing because we disabled main with #[no_main]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::testing::run_tests)]

use core::{cell::Cell, fmt::Write};

use crate::{
    os::OS,
    vga_print::{CellColor, Printer, VgaCell, VgaColor},
};

pub mod debug_exit;
pub mod os;
pub mod panic_handler;
pub mod serial_print;
pub mod testing;
pub mod vga_print;

use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};
use uart_16550::SerialPort;

lazy_static! {
    pub static ref ELYSIA_OS: Mutex<OS> = Mutex::new(OS::new(Printer::new()));
}
// Disables name mangling so the linker can recognize the entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Elysia-OS v0.1.0");

    // manually call the main function for testing because we renamed the test main function
    // because we disabled main with no main
    #[cfg(test)]
    test_main();

    loop {}
}

pub fn get_os() -> MutexGuard<'static, OS> {
    ELYSIA_OS.lock()
}
