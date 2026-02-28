// entry point for all system calls
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub extern "C" fn syscall_entry() {
    core::arch::naked_asm!(
        "swapgs",
        // Saves the userspace RSP into gs
        "mov gs:[0x8], rsp",
        // loads the stack saved in gs
        "mov rsp, gs:[0x0]",
        // Pushing arguments required for SyscallSnapshot
        "push rcx",
        "push r11",
        "push rax",
        "push rdi",
        "push rsi",
        "push rdx",
        "push r10",
        "push r8",
        "push r9",
        // 16 bits align the rsp
        "sub rsp, 8",
        "mov rdi, rsp",
        "add rdi, 8",
        "call syscall_handler",
        "add rsp, 8",
        // resume
        "pop r9",
        "pop r8",
        "pop r10",
        "pop rdx",
        "pop rsi",
        "pop rdi",
        "pop rax", // rust have modified it to be the return value
        "pop r11",
        "pop rcx",
        // Loads the userspace rsp from gs
        "mov rdx, gs:[0x8]", // 先把用户态 RSP 拿出来，暂存到 rdx
        // 按照 iretq 的要求在内核栈上压入 5 个值
        "push 0x1b", // SS (User Data)
        "push rdx",  // RSP (User RSP)
        "push r11",  // RFLAGS
        "push 0x23", // CS (User Code)
        "push rcx",  // RIP
        "swapgs",
        // resume the state
        "iretq"
    )
}
