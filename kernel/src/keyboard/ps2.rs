use lazy_static::lazy_static;
use pc_keyboard::{Keyboard, ScancodeSet1, layouts};
use spin::Mutex;
use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

lazy_static! {
    pub static ref _PS2_KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            pc_keyboard::HandleControl::Ignore
        ));
}

use crate::{
    hardware_interrupt::{HardwareInterrupt, notify_end_of_interrupt},
    keyboard::push_scancode,
};

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    notify_end_of_interrupt(HardwareInterrupt::Keyboard);

    let mut keyboard_port = Port::new(0x60);
    let scancode = unsafe { keyboard_port.read() };

    push_scancode(scancode);
}
