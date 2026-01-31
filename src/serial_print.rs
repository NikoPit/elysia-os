use core::fmt::{self, Write};

use crate::get_os;

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
    get_os()
        .serial_port
        .write_fmt(args)
        .expect("Failed to print to serial port.");
}
