use crate::{syscall, utils::SyscallResult};

pub fn change_dir(dir: *const i8, len: u64) -> SyscallResult {
    syscall!(ChangeDirectory, dir as u64, len)
}
