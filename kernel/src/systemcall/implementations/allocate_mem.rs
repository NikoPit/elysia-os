use x86_64::structures::paging::PageTableFlags;

use crate::{
    multitasking::MANAGER,
    s_println,
    systemcall::{implementations::utils::SyscallImpl, syscall_no::SyscallNo},
};

pub struct AllocMemImpl;

impl SyscallImpl for AllocMemImpl {
    const ENTRY: crate::systemcall::syscall_no::SyscallNo = SyscallNo::AllocateMem;

    fn handle_call(
        arg1: u64,
        _arg2: u64,
        _arg3: u64,
        _arg4: u64,
        _arg5: u64,
        _arg6: u64,
    ) -> Result<usize, crate::systemcall::error::SyscallError> {
        s_println!("Allocating {} pages, requested by user via arg1", arg1);
        let manager = MANAGER.lock();
        let mut current = manager.current.as_ref().unwrap().lock();

        let (mem_start, _) = current.addrspace.allocate_user(arg1);
        Ok(mem_start.as_u64() as usize)
    }
}
