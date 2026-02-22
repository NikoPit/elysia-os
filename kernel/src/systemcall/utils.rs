#[macro_export]
macro_rules! register_syscall {
    ($table: expr, $no: expr, $val: ty) => {
        $table[$no as usize] = Some(
            <$val as SyscallImpl>::handle_call
                as fn(u64, u64, u64, u64, u64, u64) -> Result<usize, SyscallError>,
        );
    };
}
