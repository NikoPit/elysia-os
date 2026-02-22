use alloc::str;

use crate::systemcall::{implementations::utils::SystemCallImpl, syscall_no::SystemCallNo};

pub struct GetTIDImpl;

impl SystemCallImpl for GetTIDImpl {
    const ENTRY: crate::systemcall::syscall_no::SystemCallNo = SystemCallNo::GetThreadID;

    fn handle_call(
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> Result<usize, crate::systemcall::error::SyscallError> {
        unimplemented!()
    }
}
