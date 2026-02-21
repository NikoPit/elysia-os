use crate::{
    register_syscall,
    systemcall::{
        error::SyscallError,
        implementations::{
            print::PrintImpl, set_fs::SetFSImpl, set_gs::SetGSImpl, utils::SystemCallImpl,
        },
        syscall_no::SystemCallNo,
    },
};

type SyscallHandler = fn(u64, u64, u64) -> Result<usize, SyscallError>;

pub static SYSCALL_TABLE: [Option<SyscallHandler>; 512] = {
    let mut table = [None; 512];

    // 编译时初始化表
    register_syscall!(table, SystemCallNo::Print, PrintImpl);
    register_syscall!(table, SystemCallNo::SetGs, SetGSImpl);
    register_syscall!(table, SystemCallNo::SetFs, SetFSImpl);

    table
};
