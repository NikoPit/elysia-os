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
        // Saves the registers into the context struct
        "mov [rdi + 8], rsp",
        "mov [rdi + 56], rbp",
        "mov [rdi + 48], rbx",
        "mov [rdi + 40], r12",
        "mov [rdi + 32], r13",
        "mov [rdi + 24], r14",
        "mov [rdi + 16], r15",
        // Updates the page table
        // No need to save the current page table
        // because the pagetable for each process
        // should always be the same
        "mov rax, [rsi]",
        "mov cr3, rax",
        // Loads the context of the next process
        "mov r15, [rsi + 16]",
        "mov r14, [rsi + 24]",
        "mov r13, [rsi + 32]",
        "mov r12, [rsi + 40]",
        "mov rbx, [rsi + 48]",
        "mov rbp, [rsi + 56]",
        // Loads the kernel stack so it wont messup the user stack
        "mov rsp, [rsi + 8]",
        // Pushes the things required for iretq
        // TODO: use ret to return to whatever it came from
        // instead of just straightup jumping to userspace with iretq
        "push [rsi + 64]", // SS
        "push [rsi + 72]", // RSP
        "push [rsi + 80]", // RFlags
        "push [rsi + 88]", // CS
        "push [rsi + 96]", // RIP
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
