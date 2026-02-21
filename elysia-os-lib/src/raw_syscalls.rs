use core::arch;

use crate::numbers::SyscallNumber;

pub fn raw_syscall_0(num: SyscallNumber) -> isize {
    unsafe {
        let mut return_value: isize;
        arch::asm!(
             "syscall",
             in("rax") num as isize,
             out("rcx") _, // syscall 会覆盖 rcx
             out("r11") _, // syscall 会覆盖 r11
              lateout("rax") return_value,
        );
        return_value
    }
}

pub fn raw_syscall_1(num: SyscallNumber, arg1: u64) -> isize {
    unsafe {
        let mut return_value: isize;
        arch::asm!(
            "syscall",
            in("rax") num as isize,
            in("rdi") arg1,
            out("rcx") _, // syscall 会覆盖 rcx
            out("r11") _, // syscall 会覆盖 r11
            lateout("rax") return_value,
        );
        return_value
    }
}

pub fn raw_syscall_2(num: SyscallNumber, arg1: u64, arg2: u64) -> isize {
    unsafe {
        let mut return_value: isize;
        arch::asm!(
              "syscall",
              in("rax") num as isize,
              in("rdi") arg1,
              in("rsi") arg2,
              out("rcx") _, // syscall 会覆盖 rcx
              out("r11") _, // syscall 会覆盖 r11
              lateout("rax") return_value,
        );
        return_value
    }
}

pub fn raw_syscall_3(num: SyscallNumber, arg1: u64, arg2: u64, arg3: u64) -> isize {
    unsafe {
        let mut return_value: isize;
        arch::asm!(
               "syscall",
               in("rax") num as isize,
               in("rdi") arg1,
               in("rsi") arg2,
               in("rdx") arg3,
               out("rcx") _, // syscall 会覆盖 rcx
               out("r11") _, // syscall 会覆盖 r11
               lateout("rax") return_value,
        );
        return_value
    }
}

pub fn raw_syscall_4(num: SyscallNumber, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> isize {
    unsafe {
        let mut return_value: isize;
        arch::asm!(
               "syscall",
               in("rax") num as isize,
               in("rdi") arg1,
               in("rsi") arg2,
               in("rdx") arg3,
               in("r10") arg4,
               out("rcx") _, // syscall 会覆盖 rcx
               out("r11") _, // syscall 会覆盖 r11
               lateout("rax") return_value,
        );
        return_value
    }
}

pub fn raw_syscall_5(
    num: SyscallNumber,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
) -> isize {
    unsafe {
        let mut return_value: isize;
        arch::asm!(
               "syscall",
               in("rax") num as isize,
               in("rdi") arg1,
               in("rsi") arg2,
               in("rdx") arg3,
               in("r10") arg4,
               in("r8") arg5,
               out("rcx") _, // syscall 会覆盖 rcx
               out("r11") _, // syscall 会覆盖 r11
               lateout("rax") return_value,
        );
        return_value
    }
}
