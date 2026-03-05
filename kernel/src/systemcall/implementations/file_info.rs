use core::str::from_utf8;

use alloc::slice;
use x86_64::structures::paging::PageTableFlags;

use crate::{
    filesystem::{path::Path, vfs::VirtualFS},
    systemcall::{implementations::utils::SyscallImpl, syscall_no::SyscallNo},
};

pub struct FileInfoImpl;

impl SyscallImpl for FileInfoImpl {
    const ENTRY: crate::systemcall::syscall_no::SyscallNo = SyscallNo::FileInfo;

    fn handle_call(
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> Result<usize, crate::systemcall::error::SyscallError> {
        let path_str =
            unsafe { from_utf8(slice::from_raw_parts(arg1 as *const u8, arg2 as usize)).unwrap() };
        let path = Path::new(path_str);

        let info = VirtualFS.lock().file_info(path).unwrap();

        Ok(0)
    }
}
