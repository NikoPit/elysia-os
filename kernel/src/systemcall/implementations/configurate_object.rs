use crate::{
    object::{config::ConfigurateRequest, get_object},
    systemcall::{implementations::utils::SyscallImpl, syscall_no::SyscallNo},
};

pub struct ConfigurateObjectImpl;

impl SyscallImpl for ConfigurateObjectImpl {
    const ENTRY: crate::systemcall::syscall_no::SyscallNo = SyscallNo::ConfigurateObject;

    fn handle_call(
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> Result<usize, crate::systemcall::error::SyscallError> {
        Ok(get_object(arg1)
            .unwrap()
            .as_configuratable()
            .unwrap()
            .configure(ConfigurateRequest::new(arg2, arg3))
            .unwrap() as usize)
    }
}
