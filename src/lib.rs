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
pub mod acpi;
pub mod gdt;
pub mod hardware_interrupt;
pub mod interrupts;
pub mod memory;
pub mod misc;
pub mod multitasking;
pub mod os;
pub mod panic_handler;
pub mod serial_print;
pub mod systemcall;
pub mod testing;
pub mod tss;
pub mod vga_print;

#[cfg(test)]
use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::BootInfo;

#[cfg(test)]
entry_point!(test_k_main);

#[cfg(test)]
fn test_k_main(_boot_info: &'static BootInfo) -> ! {
    use crate::{
        memory::paging::{BootinfoFrameAllocator, init_mapper},
        misc::hlt_loop,
        os::get_os,
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
