use core::str::from_utf8;

use crate::{
    new_syscall,
    os::get_os,
    println, s_println,
    systemcall::{
        error::SyscallError, implementations::utils::SystemCallImpl, syscall_no::SystemCallNo,
    },
};

new_syscall!(PrintImpl, SystemCallNo::Print, buf: *const u8, count: usize, empty: u64, |buf: *const u8, count: usize, empty: u64| -> Result<usize, SyscallError> {
    s_println!("{}", from_utf8(unsafe { core::slice::from_raw_parts(buf, count) }).unwrap());
    Ok(0)
});
