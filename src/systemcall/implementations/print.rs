use core::str::from_utf8;

use crate::{
    new_syscall,
    os::get_os,
    systemcall::{
        error::SyscallError, implementations::utils::SystemCallImpl, syscall_no::SystemCallNo,
    },
    vga_print::CellColor,
};

new_syscall!(PrintImpl, SystemCallNo::Print, fd: i32, buf: *const u8, count: usize, |fd, buf: *const u8, count: usize| -> Result<usize, SyscallError> {
    if fd < 0 {
        // Bad file descripter
        return Err(SyscallError::InvalidFileDescriptor);
    }

    match fd {
        1 => Ok(write_to_stdio(buf, count, false)),
        2 => Ok(write_to_stdio(buf, count, true)),
        _ => Err(SyscallError::InvalidFileDescriptor),
    }
});

fn write_to_stdio(buf: *const u8, count: usize, error: bool) -> usize {
    let test = unsafe { core::slice::from_raw_parts(buf, count) };
    let test = from_utf8(test).unwrap();
    unsafe {
        get_os().printer.print_string(
            test,
            CellColor::new(
                if error {
                    crate::vga_print::VgaColor::Red
                } else {
                    crate::vga_print::VgaColor::White
                },
                crate::vga_print::VgaColor::Black,
            ),
        )
    };

    0
}
