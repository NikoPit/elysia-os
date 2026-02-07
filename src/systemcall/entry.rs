
// entry point for all system calls
#[unsafe(no_mangle)]
pub extern "C" fn syscall_entry() {
    unsafe {
        core::arch::asm!(
            // Pushing arguments required for SyscallSnapshot
            "push rcx",
            "push r11",
            "push rax",
            "push rdi",
            "push rsi",
            "push rdx",
            // rsp: stack pointer
            // rdi: 1st argument
            // calls syscall_handler with the things
            // we've pushed to the stack
            "mov rdi, rsp",
            "call syscall_handler",
            // resume
            "pop rdx",
            "pop rsi",
            "pop rdi",
            "pop rax", // rust have modified it to be the return value
            "pop r11",
            "pop rcx",
            // resume the state
            "sysret"
        )
    };
}
