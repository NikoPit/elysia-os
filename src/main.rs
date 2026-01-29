// Disable dynamic linking with the std library because there is no std library in our own os
#![no_std]
// Disables main function to customize entry point
#![no_main]

use core::cell::Cell;

use crate::vga_print::{CellColor, Printer, VgaCell, VgaColor};

pub mod panic_handler;
pub mod vga_print;

static HELLO: &[u8] = b"Hello World!";

// Disables name mangling so the linker can recognize the entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    Printer::new().print_string(
        "weg沃尔夫看哦沃尔夫",
        CellColor::new(VgaColor::Black, VgaColor::White),
    );

    loop {}
}
