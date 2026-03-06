use crate::{
    multitasking::MANAGER,
    systemcall::{implementations::utils::SyscallImpl, syscall_no::SyscallNo},
};

pub struct ForkImpl;

impl SyscallImpl for ForkImpl {
    const ENTRY: crate::systemcall::syscall_no::SyscallNo = SyscallNo::Fork;

    fn handle_call(
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> Result<usize, crate::systemcall::error::SyscallError> {
        let manager = MANAGER.lock();

        manager.current.clone().unwrap().lock().fork();

        Ok(0)
    }
}
