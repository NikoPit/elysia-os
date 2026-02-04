use crate::systemcall::call_entries::SystemCallEntry;

pub mod write;

pub trait SystemCallImpl {
    const ENTRY: SystemCallEntry;

    fn handle_call(arg1: u64, arg2: u64, arg3: u64) -> isize;
}
