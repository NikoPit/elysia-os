#![no_std]
// Disables main function to customize entry point
#![no_main]
#![feature(abi_x86_interrupt, custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(kernel::testing::run_tests)]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use kernel::{
    debug_exit::debug_exit, init, misc::hlt_loop, panic_handler::test_handle_panic, s_println,
};

entry_point!(k_main);

fn k_main(bootinfo: &'static BootInfo) -> ! {
    init(bootinfo);

    s_println!("todo");
    debug_exit(kernel::debug_exit::QemuExitCode::Success);

    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_handle_panic(info)
}
