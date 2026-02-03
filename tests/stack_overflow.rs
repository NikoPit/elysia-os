#![no_std]
// Disables main function to customize entry point
#![no_main]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
// renames main function for testing because we disabled main with #[no_main]
#![reexport_test_harness_main = "test_main"]
#![test_runner(testing::run_tests)]
use bootloader::BootInfo;
use bootloader::entry_point;
use elysia_os::debug_exit::debug_exit;
use elysia_os::os::get_os;
use elysia_os::paging::BootinfoFrameAllocator;
use elysia_os::paging::init_mapper;
use elysia_os::print;
use elysia_os::s_print;
use elysia_os::s_println;
use elysia_os::testing;
use volatile::VolatilePtr;
use volatile::VolatileRef;
use x86_64::instructions::interrupts::int3;
// Disable dynamic linking with the std library because there is no std library in our own os

use core::panic::PanicInfo;

use elysia_os::panic_handler;
use elysia_os::panic_handler::test_handle_panic;
use elysia_os::println;
use elysia_os::test;

entry_point!(_start);
fn _start(bootinfo: &'static BootInfo) -> ! {
    s_print!("\nStack overflow double-fault handling ");
    let mut frame_allocator: BootinfoFrameAllocator =
        unsafe { BootinfoFrameAllocator::new(&bootinfo.memory_map) };
    let mut mapper = init_mapper(bootinfo);

    get_os().init(bootinfo, &mut mapper, &mut frame_allocator);

    stack_overflow();

    s_println!("[FAILED]\n");
    s_println!("Test continued to run after stack overflow\n");
    debug_exit(elysia_os::debug_exit::QemuExitCode::Failed);

    loop {}
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    s_println!("[OK]\n");
    s_println!("Test success!");
    debug_exit(elysia_os::debug_exit::QemuExitCode::Success);
    loop {}
}
