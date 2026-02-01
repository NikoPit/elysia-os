use core::fmt::{self, Write};

use crate::os::{get_os, get_os_no_interrupt};

#[macro_export]
macro_rules! s_print {
    ($($arg:tt)*) => ($crate::serial_print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! s_println {
    () => ($crate::s_print!("\n"));
    ($($arg:tt)*) => ($crate::s_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    get_os_no_interrupt(|mut os| {
        os.serial_port
            .write_fmt(args)
            .expect("Failed to print to serial port")
    });
}
