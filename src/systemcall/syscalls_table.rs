use crate::{
    register_syscall,
    systemcall::{
        error::SyscallError,
        implementations::{utils::SystemCallImpl, write::WriteImpl},
        syscall_no::SystemCallNo,
    },
};

type SyscallHandler = fn(u64, u64, u64) -> Result<usize, SyscallError>;

pub static SYSCALL_TABLE: [Option<SyscallHandler>; 512] = {
    let mut table = [None; 512];

    // 编译时初始化表
    register_syscall!(table, SystemCallNo::Write, WriteImpl);

    table
};
