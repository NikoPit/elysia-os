use x86_64::{
    instructions::interrupts,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{
    driver::init_interrupt_drivers, exception_interrupt::init_exception_interrupts,
    hardware_interrupt::init_hardware_interrupts, os::get_os, print, println, test,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        init_hardware_interrupts(&mut idt);
        init_interrupt_drivers(&mut idt);
        init_exception_interrupts(&mut idt);

        idt
    };
}

pub fn init() {
    IDT.load();
    unsafe { get_os().pics.initialize() };
    interrupts::enable();
}

pub fn print_stackframe_m(stack_frame: InterruptStackFrame) {
    println!("{:#?}", stack_frame);
}

pub fn print_stackframe(message: &str, stack_frame: InterruptStackFrame) {
    print!("\n{message}:\n\n");
    print_stackframe_m(stack_frame);
}

// test if breakpoint interrupt will crash the system
test!("Breakpoint interrupt crash", || int3());
