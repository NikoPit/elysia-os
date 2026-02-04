pub enum SyscallError {
    InvalidSyscall = -38,
    InvalidFileDescriptor = -255,
    Other = -256,
}
