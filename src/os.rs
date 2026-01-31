use core::fmt::Write;

use uart_16550::SerialPort;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::{interrupts::init_idt, vga_print::Printer};

pub struct OS {
    pub printer: Printer,
    pub serial_port: SerialPort,
}

impl OS {
    pub fn new() -> Self {
        Self {
            printer: Printer::new(),
            serial_port: {
                let mut serial_port = unsafe { SerialPort::new(0x3F8) };
                serial_port.init();
                serial_port
            },
        }
    }

    pub fn init(&self) {
        init_idt();
    }
}
