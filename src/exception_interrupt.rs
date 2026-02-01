use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{
    interrupts::{print_stackframe, print_stackframe_m},
    print,
    tss::DOUBLE_FAULT_IST_LOCATION,
};

pub trait ExceptionInterruptHandler {
    fn handle_exception_interrupt_unwrapped(_stack_frame: InterruptStackFrame) {}
    fn handle_exception_interrupt_unwrapped_err_code(
        _stack_frame: InterruptStackFrame,
        err_code: u64,
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
