// Snapshot of the operating system. Including registers.
// Also known as Frame, Context, etc.

use core::arch::naked_asm;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Snapshot {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rax: u64,

    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

#[macro_export]
macro_rules! load_registers {
    ($base:literal) => {
        concat!(
            "mov r15, [",
            $base,
            " + 0]",
            "mov r14, [",
            $base,
            " + 8]",
            "mov r13, [",
            $base,
            " + 16]",
            "mov r12, [",
            $base,
            " + 24]",
            "mov r11, [",
            $base,
            " + 32]",
            "mov r10, [",
            $base,
            " + 40]",
            "mov r9,  [",
            $base,
            " + 48]",
            "mov r8,  [",
            $base,
            " + 56]",
            "mov rsi, [",
            $base,
            " + 72]", // 跳过 rdi (offset 64)
            "mov rbp, [",
            $base,
            " + 80]",
            "mov rbx, [",
            $base,
            " + 88]",
            "mov rdx, [",
            $base,
            " + 96]",
            "mov rcx, [",
            $base,
            " + 104]",
            "mov rax, [",
            $base,
            " + 112]",
            "mov rdi, [",
            $base,
            " + 64]", // 最后恢复基址寄存器
        )
    };
}

impl Snapshot {
    pub fn default_regs(rip: u64, cs: u16, rflags: u64, rsp: u64, ss: u16) -> Self {
        Self {
            rip,
            cs: cs as u64,
            rflags,
            rsp,
            ss: ss as u64,
            ..Default::default()
        }
    }
}
