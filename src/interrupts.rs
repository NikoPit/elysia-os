use x86_64::{
    instructions::interrupts::int3,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{print, println, test, tss::DOUBLE_FAULT_IST_LOCATION};
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(doublefault_handler)
                // sets the custom stack for double fault to prevent not being able
                // to handle double fault when stack overflows
                .set_stack_index(DOUBLE_FAULT_IST_LOCATION)
        };

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

pub extern "x86-interrupt" fn doublefault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "Double fault:\n\n{:#?}\nError code: {error_code}",
        stack_frame
    );
}

// test if breakpoint interrupt will crash the system
test!("Breakpoint interrupt crash", || int3());
