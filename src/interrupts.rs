use x86_64::{
    instructions::interrupts::int3,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{print, println, test};
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

fn print_stackframe_m(stack_frame: InterruptStackFrame) {
    println!("{:#?}", stack_frame);
}

fn print_stackframe(message: &str, stack_frame: InterruptStackFrame) {
    print!("\n{message}:\n\n");
    print_stackframe_m(stack_frame);
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    print_stackframe("breakpoint exception", stack_frame);
}

pub extern "x86-interrupt" fn doublefault_handler(stack_frame: InterruptStackFrame) {
    print_stackframe("double fault", stack_frame);
}

// test if breakpoint interrupt will crash the system
test!("Breakpoint interrupt crash", || int3());
