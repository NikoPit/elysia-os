use crate::systemcall::implementations::{SystemCallEntry, SystemCallImpl};

pub struct WriteCall;

impl SystemCallImpl for WriteCall {
    const ENTRY: super::SystemCallEntry = SystemCallEntry::Write;

    fn handle_call(arg1: u64, arg2: u64, arg3: u64) -> isize {
        0
    }
}
