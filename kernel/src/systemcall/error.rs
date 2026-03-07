pub enum SyscallError {
    BufferTooSmall = -1,
    InvalidPath = -2,
    InvalidString = -3,
    InvalidSyscall = -38,
    UnconfiguratableObject = -400,
    InvalidFileDescriptor = -255,
    Other = -256,
}
