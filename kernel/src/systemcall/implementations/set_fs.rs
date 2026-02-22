use x86_64::registers::model_specific::Msr;

use crate::{
    new_syscall,
    systemcall::{error::SyscallError, implementations::utils::SyscallImpl, syscall_no::SyscallNo},
};

pub struct SetFSImpl;

impl SyscallImpl for SetFSImpl {
    const ENTRY: SyscallNo = SyscallNo::SetFs;

    fn handle_call(
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> Result<usize, SyscallError> {
        unsafe {
            Msr::new(0xC0000100).write(arg1);
        };
        Ok(0)
    }
}
