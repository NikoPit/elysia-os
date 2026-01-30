use core::fmt::Write;

use uart_16550::SerialPort;

use crate::vga_print::Printer;

pub struct OS {
    pub printer: Printer,
    pub serial_port: SerialPort,
}

impl OS {
    pub fn new(printer: Printer) -> Self {
        Self {
            printer,
            serial_port: {
                let mut serial_port = unsafe { SerialPort::new(0x3F8) };
                serial_port.init();
                serial_port
            },
        }
    }
}
