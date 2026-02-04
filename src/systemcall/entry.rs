use x86_64::{
    VirtAddr,
    instructions::interrupts::without_interrupts,
    registers::{
        control::{Efer, EferFlags},
        model_specific::{LStar, SFMask},
        rflags::RFlags,
    },
};
pub fn init_syscall() {
    without_interrupts(|| {
        // enable systemcalls
        unsafe { Efer::update(|efer| efer.insert(EferFlags::SYSTEM_CALL_EXTENSIONS)) };

        // disable interrupts on systemcalls
        SFMask::write(RFlags::INTERRUPT_FLAG);

        // sets the entry point for systemcalls
        let syscall_entry_addr = VirtAddr::new(syscall_entry as usize as u64);
        LStar::write(syscall_entry_addr);
    })
}

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
