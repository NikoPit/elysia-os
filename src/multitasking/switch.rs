use core::arch;

use crate::multitasking::{self, manager::Manager};

impl Manager {
    // NOTE: DO NOT call context_switch deep within a call stack
    // because it will messup the stack completely
    /// # Safety
    /// Must provide valid current / next conteext ptr
    #[unsafe(naked)]
    pub unsafe extern "C" fn context_switch(
        current: *mut multitasking::context::Context,
        next: *mut multitasking::context::Context,
    ) {
        arch::naked_asm!(
            // Constructs the "Context"
            // Note: the instruction pointer have already been automatically
            // saved when we call switch();
            // because of this, we dont have to explicitly save it
            "push rbp",
            "push rbx",
            "push r12",
            "push r13",
            "push r14",
            "push r15",
            "pushfq",
            // Updates the context of the current process
            // to the context we've pushed to the stack
            // note to future me: the [] basically means refrence
            // &current (1st argument) = rsp (stack pointer);
            // this also saves the rsp into the rsp value of the context struct (i think lol)
            "mov [rdi + 8], rsp",
            // Updates the page table
            "mov rax, [rsi]",
            "mov cr3, rax",
            // rsp = &next; (rsi = second arg) (updates the stack with the context of the next process)
            "mov rsp, [rsi + 8]",
            // pop the context of the next process
            // back into the cpu registers
            "popfq",
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop rbx",
            "pop rbp",
            // Go back to the instruction pointer that the process is at
            // (its on the stack top right now so RET will go there)
            // We dont have to explicitly push it or something
            // becuase its already been saved when we called
            // switch. so now its already sitting at the stack top
            "ret"
        )
    }

    pub unsafe extern "C" fn context_switch_zombie(next: *mut multitasking::context::Context) {
        arch::asm!(
            // rsp = &next; (rsi = second arg) (updates the stack with the context of the next process)
            "mov rsp, [rdi]"
        );
        arch::asm!(
            // pop the context of the next process
            // back into the cpu registers
            "popfq", "pop r15", "pop r14", "pop r13", "pop r12", "pop rbx", "pop rbp"
        );
        // Go back to the instruction pointer that the process is at
        // (its on the stack top right now so RET will go there)
        // We dont have to explicitly push it or something
        // becuase its already been saved when we called
        // switch. so now its already sitting at the stack top
        arch::asm!("ret", options(noreturn));
    }
}
