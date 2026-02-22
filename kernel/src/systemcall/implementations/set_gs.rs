use crate::systemcall::{implementations::utils::SystemCallImpl, syscall_no::SystemCallNo};

pub struct SetGSImpl;

impl SystemCallImpl for SetGSImpl {
    const ENTRY: crate::systemcall::syscall_no::SystemCallNo = SystemCallNo::SetGs;

    fn handle_call(
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> Result<usize, crate::systemcall::error::SyscallError> {
        Err(crate::systemcall::error::SyscallError::Other)
    }
}
