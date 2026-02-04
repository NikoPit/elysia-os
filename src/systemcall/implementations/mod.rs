use crate::systemcall::syscall_no::SystemCallNo;

pub mod write;

pub trait SystemCallImpl {
    const ENTRY: SystemCallNo;

    fn handle_call(arg1: u64, arg2: u64, arg3: u64) -> isize;
}
