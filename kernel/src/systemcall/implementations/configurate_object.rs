use crate::{
    object::{config::ConfigurateRequest, get_object},
    println,
    systemcall::{error::SyscallError, implementations::utils::SyscallImpl, syscall_no::SyscallNo},
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
        let res = get_object(arg1)
            .ok_or(SyscallError::InvalidFileDescriptor)?
            .as_configuratable()
            .ok_or(SyscallError::UnconfiguratableObject)?
            .configure(ConfigurateRequest::new(arg2, arg3));

        match res {
            Ok(val) => Ok(val as usize),
            Err(_) => {
                println!("Failed when trying to configurate object. returnning Ok(0)");
                Ok(0)
            }
        }
    }
}
