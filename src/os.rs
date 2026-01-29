use core::fmt::Write;

use crate::vga_print::Printer;

pub struct OS {
    pub printer: Printer,
}

impl OS {
    pub fn new(printer: Printer) -> Self {
        Self { printer }
    }
}
