use x86_64::structures::paging::PageTableFlags;

use crate::{
    memory::manager::allocate_user_mem,
    multitasking::MANAGER,
    systemcall::{implementations::utils::SystemCallImpl, syscall_no::SystemCallNo},
};

pub struct AllocMemImpl;

impl SystemCallImpl for AllocMemImpl {
    const ENTRY: crate::systemcall::syscall_no::SystemCallNo = SystemCallNo::AllocateMem;

    fn handle_call(
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> Result<usize, crate::systemcall::error::SyscallError> {
        let mut manager = MANAGER.lock();
        Ok(allocate_user_mem(
            arg1,
            &mut manager.get_current().page_table.inner,
            PageTableFlags::from_bits(arg2).unwrap(),
        )
        .0
        .as_u64() as usize)
    }
}
