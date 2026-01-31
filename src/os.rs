use lazy_static::lazy_static;
use spin::*;
use uart_16550::SerialPort;

use crate::{gdt::init_gdt, interrupts::init_idt, vga_print::Printer};

lazy_static! {
    pub static ref ELYSIA_OS: Mutex<OS> = Mutex::new(OS::new());
}

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
        init_gdt();
        init_idt();
    }
}

pub fn get_os() -> MutexGuard<'static, OS> {
    ELYSIA_OS.lock()
}
