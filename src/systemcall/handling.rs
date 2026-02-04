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
        call_entries::SystemCallEntry, error::SyscallError, implementations::SystemCallImpl,
        implementations::write::WriteCall,
    },
};

#[repr(C)]
struct SyscallSnapshot {
    arg3: u64,         // rdx
    arg2: u64,         // rsi
    arg1: u64,         // rdi
    call_entry: isize, // rax
    // required for sysret to correctly resume (go back to the previous instruction)
    rflags: u64, // r11
    rip: u64,    // rcx
}

#[unsafe(no_mangle)]
pub extern "C" fn syscall_handler(snapshot_ptr: *mut SyscallSnapshot) {
    let snapshot = unsafe { &mut *snapshot_ptr };

    println!("Called!");
    println!("{}", snapshot.call_entry);

    let result = syscall_handler_unwrapped(
        snapshot.call_entry,
        snapshot.arg1,
        snapshot.arg2,
        snapshot.arg3,
    );

    snapshot.call_entry = result;
}

fn syscall_handler_unwrapped(call_entry: isize, arg1: u64, arg2: u64, arg3: u64) -> isize {
    // Check if the entry exists
    let entry = match SystemCallEntry::try_from(call_entry) {
        Ok(value) => value,
        Err(_) => return SyscallError::DoesntExist as isize,
    };

    match entry {
        SystemCallEntry::Write => WriteCall::handle_call(arg1, arg2, arg3),
    }
}
