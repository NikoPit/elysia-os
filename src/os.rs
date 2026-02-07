use crate::{
    hardware_interrupt::{PIC_1_OFFSET, PIC_2_OFFSET},
    vga_print::Printer,
};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::*;
use uart_16550::SerialPort;
use x86_64::{
    VirtAddr,
    instructions::interrupts::{self},
};

lazy_static! {
    pub static ref ELYSIA_OS: Mutex<OS> = Mutex::new(OS::new());
}

pub struct OS {
    pub printer: Printer,
    pub serial_port: SerialPort,
    pub pics: ChainedPics,
    pub phys_mem_offset: Option<VirtAddr>,
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
            phys_mem_offset: None,
        }
    }
}

pub fn get_os_no_interrupt<F>(func: F)
where
    F: FnOnce(MutexGuard<'static, OS>),
{
    interrupts::without_interrupts(|| func(get_os()));
}

pub fn get_os() -> MutexGuard<'static, OS> {
    ELYSIA_OS.lock()
}
