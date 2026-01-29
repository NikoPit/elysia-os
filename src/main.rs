// Disable dynamic linking with the std library because there is no std library in our own os
#![no_std]
// Disables main function to customize entry point
#![no_main]

use core::{cell::Cell, fmt::Write};

use crate::{
    os::OS,
    vga_print::{CellColor, Printer, VgaCell, VgaColor},
};

pub mod os;
pub mod panic_handler;
pub mod vga_print;

use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

lazy_static! {
    pub static ref ELYSIA_OS: Mutex<OS> = Mutex::new(OS::new(Printer::new()));
}
// Disables name mangling so the linker can recognize the entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("hello world!");

    loop {}
}

pub fn get_os() -> MutexGuard<'static, OS> {
    ELYSIA_OS.lock()
}
