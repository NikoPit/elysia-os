use crate::systemcall::implementations::{SystemCallImpl, SystemCallNo};

pub struct WriteCall;

impl SystemCallImpl for WriteCall {
    const ENTRY: super::SystemCallNo = SystemCallNo::Write;

    fn handle_call(fd: u64, arg2: u64, arg3: u64) -> isize {
        0
    }
}
