use x86_64::{
    VirtAddr,
    instructions::interrupts::without_interrupts,
    registers::{
        control::{Efer, EferFlags},
        model_specific::{LStar, SFMask},
        rflags::RFlags,
    },
};

use crate::{
    println,
    systemcall::{
        error::SyscallError, implementations::SystemCallImpl, implementations::write::WriteCall,
        syscall_no::SystemCallNo,
    },
};

#[repr(C)]
struct SyscallSnapshot {
    arg3: u64,         // rdx
    arg2: u64,         // rsi
    arg1: u64,         // rdi
    syscall_no: isize, // rax
    // required for sysret to correctly resume (go back to the previous instruction)
    rflags: u64, // r11
    rip: u64,    // rcx
}

#[unsafe(no_mangle)]
pub extern "C" fn syscall_handler(snapshot_ptr: *mut SyscallSnapshot) {
    let snapshot = unsafe { &mut *snapshot_ptr };

    println!("Called!");
    println!("{}", snapshot.syscall_no);

    let result = syscall_handler_unwrapped(
        snapshot.syscall_no,
        snapshot.arg1,
        snapshot.arg2,
        snapshot.arg3,
    );

    snapshot.syscall_no = result;
}

fn syscall_handler_unwrapped(syscall_no: isize, arg1: u64, arg2: u64, arg3: u64) -> isize {
    // Check if the entry exists
    let entry = match SystemCallNo::try_from(syscall_no) {
        Ok(value) => value,
        Err(_) => return SyscallError::DoesntExist as isize,
    };

    match entry {
        SystemCallNo::Write => WriteCall::handle_call(arg1, arg2, arg3),
    }
}
