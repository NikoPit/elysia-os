pub enum SyscallError {
    InvalidSyscall = -38,
    UnconfiguratableObject = -400,
    InvalidFileDescriptor = -255,
    Other = -256,
}
