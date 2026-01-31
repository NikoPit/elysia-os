use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::*;
use uart_16550::SerialPort;
use x86_64::instructions::interrupts;

use crate::{
    gdt::init_gdt,
    hardware_interrupt::{PIC_1_OFFSET, PIC_2_OFFSET},
    interrupts::init_idt,
    vga_print::Printer,
};

lazy_static! {
    pub static ref ELYSIA_OS: Mutex<OS> = Mutex::new(OS::new());
}

pub struct OS {
    pub printer: Printer,
    pub serial_port: SerialPort,
    pub pics: ChainedPics,
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
            pics: unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) },
        }
    }

    pub fn init(&mut self) {
        init_gdt();
        init_idt();

        unsafe { self.pics.initialize() };
        interrupts::enable();
    }
}

pub fn get_os() -> MutexGuard<'static, OS> {
    ELYSIA_OS.lock()
}
