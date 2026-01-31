use x86_64::{
    instructions::interrupts::int3,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{println, test};
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("\nBreakpoint exception:\n\n{:#?}", stack_frame);
}

// test if breakpoint interrupt will crash the system
test!("Breakpoint interrupt crash", || int3());
