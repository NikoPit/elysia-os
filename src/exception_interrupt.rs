use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

use crate::{
    interrupts::{print_stackframe, print_stackframe_m},
    misc::hlt_loop, println,
    tss::DOUBLE_FAULT_IST_LOCATION,
};

pub trait ExceptionInterruptHandler {
    fn handle_exception_interrupt_unwrapped(_stack_frame: InterruptStackFrame) {}
    fn handle_exception_interrupt_unwrapped_err_code(
        _stack_frame: InterruptStackFrame,
        _err_code: u64,
    ) -> ! {
        unimplemented!();
    }

    extern "x86-interrupt" fn handle_exception_interrupt(_stack_frame: InterruptStackFrame) {
        Self::handle_exception_interrupt_unwrapped(_stack_frame);
    }

    extern "x86-interrupt" fn handle_exception_interrupt_err_code(
        _stack_frame: InterruptStackFrame,
        err_code: u64,
    ) -> ! {
        Self::handle_exception_interrupt_unwrapped_err_code(_stack_frame, err_code);
    }
}

pub fn init_exception_interrupts(idt: &mut InterruptDescriptorTable) {
    idt.breakpoint
        .set_handler_fn(BreakpointHandler::handle_exception_interrupt);
    unsafe {
        idt.double_fault
            .set_handler_fn(DoublefaultHandler::handle_exception_interrupt_err_code)
            .set_stack_index(DOUBLE_FAULT_IST_LOCATION)
    };
    idt.page_fault.set_handler_fn(pagefault_handler);
}

struct BreakpointHandler;
struct DoublefaultHandler;

impl ExceptionInterruptHandler for BreakpointHandler {
    fn handle_exception_interrupt_unwrapped(_stack_frame: InterruptStackFrame) {
        print_stackframe("Breakpoint exception:\n", _stack_frame);
    }
}

impl ExceptionInterruptHandler for DoublefaultHandler {
    fn handle_exception_interrupt_unwrapped_err_code(
        _stack_frame: InterruptStackFrame,
        err_code: u64,
    ) -> ! {
        panic!(
            "Double fault:\n\n{:#?}\nError code: {err_code}",
            _stack_frame
        );
    }
}

// i gave up on trying to wrap everything behind a abstraction layer.
extern "x86-interrupt" fn pagefault_handler(
    _stack_frame: InterruptStackFrame,
    err_code: PageFaultErrorCode,
) {
    println!("Page fault");
    println!("Adress: {:?}", Cr2::read());
    println!("Error code: {:?}", err_code);
    print_stackframe_m(_stack_frame);
    hlt_loop();
}
