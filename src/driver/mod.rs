pub mod keyboard;

use core::char;

use pc_keyboard::{DecodedKey, KeyCode, Keyboard, ScancodeSet1, layouts};
use x86_64::{instructions::port::Port, structures::idt::InterruptDescriptorTable};

use crate::{
    driver::keyboard::ps2::PS2KeyboardDriver,
    hardware_interrupt::{HardwareInterrupt, HardwareInterruptHandler},
    os::get_os,
    panic_handler::handle_panic,
    print, println,
};

pub trait Driver {}

pub trait InterruptDriver: Driver {
    fn idt_init(idt: &mut InterruptDescriptorTable);
}

pub fn init_interrupt_drivers(idt: &mut InterruptDescriptorTable) {
    PS2KeyboardDriver::idt_init(idt);
}
