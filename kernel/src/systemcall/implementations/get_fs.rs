use x86_64::registers::model_specific::Msr;

use crate::systemcall::{
    error::SyscallError, implementations::utils::SystemCallImpl, syscall_no::SystemCallNo,
};

pub struct GetFSImpl;

impl SystemCallImpl for GetFSImpl {
    const ENTRY: SystemCallNo = SystemCallNo::GetFs;
    fn handle_call(arg1: u64, arg2: u64, arg3: u64) -> Result<usize, SyscallError> {
        unsafe { Ok(Msr::new(0xC0000100).read() as usize) }
    }
}
