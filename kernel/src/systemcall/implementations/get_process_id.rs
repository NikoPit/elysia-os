use alloc::str;

use crate::{
    multitasking::MANAGER,
    systemcall::{implementations::utils::SystemCallImpl, syscall_no::SystemCallNo},
};

pub struct GetPIDImpl;

impl SystemCallImpl for GetPIDImpl {
    const ENTRY: crate::systemcall::syscall_no::SystemCallNo = SystemCallNo::GetProcessID;

    fn handle_call(
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> Result<usize, crate::systemcall::error::SyscallError> {
        Ok(MANAGER
            .lock()
            .current
            .expect("Theres no current process. WHAT? HOW?")
            .0 as usize)
    }
}
