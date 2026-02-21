use crate::{errors::SyscallError, numbers::SyscallNumber, syscall, utils::SyscallResult};

#[inline(always)]
pub fn print(value: &str) -> SyscallResult {
    let msg = value.as_bytes();
    let buf = msg.as_ptr();
    let count = msg.len();

    syscall!(Print, buf as u64, count as u64)
}

#[inline]
pub fn print_buf(buf: &[u8], len: u64) -> SyscallResult {
    let buf = buf.as_ptr();

    syscall!(Print, buf as u64, len)
}

pub fn set_fs(addr: u64) -> SyscallResult {
    syscall!(SetFs, addr)
}

pub fn get_fs() -> SyscallResult {
    syscall!(GetFs)
}

pub fn set_gs(addr: u64) -> SyscallResult {
    syscall!(SetGs, addr)
}
