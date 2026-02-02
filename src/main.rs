#![no_std]
// Disables main function to customize entry point
#![no_main]
#![feature(abi_x86_interrupt, custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(elysia_os::testing::run_tests)]
// renames main function for testing because we disabled main with #[no_main]
// Disable dynamic linking with the std library because there is no std library in our own os

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use elysia_os::misc::hlt_loop;
use elysia_os::panic_handler::handle_panic;
use elysia_os::{os::get_os, println};

entry_point!(k_main);

fn k_main(bootinfo: &'static BootInfo) -> ! {
    println!("Welcome to Elysia-OS v0.1.0");

    get_os().init();

    #[cfg(test)]
    test_main();

    hlt_loop();
}

pub fn run_tests(tests: &[&dyn Fn()]) {
    use elysia_os::{
        debug_exit::{QemuExitCode, debug_exit},
        s_println,
    };

    s_println!("\nRunning {} tests", tests.len());
    for test in tests {
        test();
    }

    s_println!("\nTest success!");
    debug_exit(QemuExitCode::Success);
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    test_handle_panic(_info);
    use elysia_os::panic_handler::test_handle_panic;
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    handle_panic(_info);
}
