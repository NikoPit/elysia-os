use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;
use crate::s_println;
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
