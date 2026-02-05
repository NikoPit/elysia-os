#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::testing::run_tests)]

extern crate alloc;

pub mod debug_exit;
pub mod driver;
pub mod exception_interrupt;
//pub mod filesystem;
pub mod gdt;
pub mod hardware_interrupt;
pub mod heap;
pub mod heap_allocators;
pub mod interrupts;
pub mod misc;
pub mod multitasking;
pub mod os;
pub mod paging;
pub mod panic_handler;
pub mod serial_print;
pub mod systemcall;
pub mod testing;
pub mod tss;
pub mod vga_print;

#[cfg(test)]
use core::panic::PanicInfo;

use alloc::boxed::Box;
#[cfg(test)]
use bootloader::BootInfo;
use bootloader::entry_point;

#[cfg(test)]
entry_point!(test_k_main);

#[cfg(test)]
fn test_k_main(_boot_info: &'static BootInfo) -> ! {
    use crate::{
        misc::hlt_loop,
        os::get_os,
        paging::{BootinfoFrameAllocator, init_mapper},
    };
    let mut frame_allocator: BootinfoFrameAllocator =
        unsafe { BootinfoFrameAllocator::new(&_boot_info.memory_map) };
    let mut mapper = init_mapper(_boot_info);

    get_os().init(_boot_info, &mut mapper, &mut frame_allocator);

    test_main();

    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use crate::panic_handler::test_handle_panic;

    test_handle_panic(_info);
}
