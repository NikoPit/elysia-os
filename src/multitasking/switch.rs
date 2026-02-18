use core::arch;

use crate::multitasking::{self, context::Context, manager::Manager};

// NOTE: DO NOT call context_switch deep within a call stack
// because it will messup the stack completely
//
/// # Safety
/// Must provide valid current / next conteext ptr
#[unsafe(naked)]
pub unsafe extern "C" fn context_switch(current: *mut Context, next: *mut Context) {
    arch::naked_asm!(
        // Note: the instruction pointer have already been automatically
        // saved when we call switch();
        // because of this, we dont have to explicitly save it
        // Another note to future me:
        // if you dont know what [rdi+56] or whatever means
        // just ask ai lol
        //
        // Saves the registers into the context struct
        "mov [rdi + 56], rbp",
        "mov [rdi + 48], rbx",
        "mov [rdi + 40], r12",
        "mov [rdi + 32], r13",
        "mov [rdi + 24], r14",
        "mov [rdi + 16], r15",
        // Pushes the rflags
        // note to future me: the [] basically means refrence
        // Updates the rsp of the context of the current process
        // current.rsp = rsp;
        "mov [rdi + 8], rsp",
        // Updates the page table
        // No need to save the current page table
        // because the pagetable for each process
        // should always be the same
        "mov rax, [rsi]",
        "mov cr3, rax",
        // rsp = &next.rsp; (rsi = second arg)
        "mov rsp, [rsi + 8]",
        "mov r15, [rsi + 16]",
        "mov r14, [rsi + 24]",
        "mov r13, [rsi + 32]",
        "mov r12, [rsi + 40]",
        "mov rbx, [rsi + 48]",
        "mov rbp, [rsi + 56]",
        // Go back to the instruction pointer that the process is at
        // (its on the stack top right now so RET will go there)
        // We dont have to explicitly push it or something
        // becuase its already been saved when we called
        // switch. so now its already sitting at the stack top
        "iretq"
    )
}

/// # Safety
/// Must provide valid pointer to context
#[unsafe(naked)]
pub unsafe extern "C" fn context_switch_zombie(next: *mut Context) {
    arch::naked_asm!(
        "mov rax, [rdi]",
        "mov cr3, rax",
        "mov rsp, [rdi + 8]",
        "popfq",
        "mov r15, [rdi + 16]",
        "mov r14, [rdi + 24]",
        "mov r13, [rdi + 32]",
        "mov r12, [rdi + 40]",
        "mov rbx, [rdi + 48]",
        "mov rbp, [rsi + 56]",
        "ret",
    );
}
