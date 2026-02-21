use x86_64::registers::model_specific::Msr;

use crate::{
    new_syscall,
    systemcall::{
        error::SyscallError, implementations::utils::SystemCallImpl, syscall_no::SystemCallNo,
    },
};

pub struct SetFSImpl;

impl SystemCallImpl for SetFSImpl {
    const ENTRY: SystemCallNo = SystemCallNo::SetFs;
    fn handle_call(arg1: u64, arg2: u64, arg3: u64) -> Result<usize, SyscallError> {
        unsafe {
            Msr::new(0xC0000100).write(arg1);
        };
        Ok(0)
    }
}
