#![no_std]

use core::panic::PanicInfo;

use crate::syscalls::{object::write_object, print};

pub mod c_wrapper;
pub mod errors;
pub mod numbers;
pub mod syscalls;
pub mod utils;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print("elyisaos-syslib panic").unwrap();
    loop {}
}
