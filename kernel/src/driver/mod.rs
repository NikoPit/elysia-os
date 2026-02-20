pub mod keyboard;


use x86_64::structures::idt::InterruptDescriptorTable;

use crate::driver::keyboard::ps2::PS2KeyboardDriver;

pub trait Driver {}

pub trait InterruptDriver: Driver {
    fn idt_init(idt: &mut InterruptDescriptorTable);
}

pub fn init_interrupt_drivers(idt: &mut InterruptDescriptorTable) {
    PS2KeyboardDriver::idt_init(idt);
}
