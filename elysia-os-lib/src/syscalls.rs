use crate::{errors::SyscallError, numbers::SyscallNumber, syscall, utils::SyscallResult};

pub fn print(value: &str) -> SyscallResult {
    let msg = value.as_bytes();
    let buf = msg.as_ptr();
    let count = msg.len();

    syscall!(Print, buf as u64, count as u64)
}

pub fn set_fs(addr: u64) -> SyscallResult {
    syscall!(SetFs, addr)
}

pub fn set_gs(addr: u64) -> SyscallResult {
    syscall!(SetGs, addr)
}
