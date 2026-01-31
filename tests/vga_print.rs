#![no_std]
// Disables main function to customize entry point
#![no_main]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
// renames main function for testing because we disabled main with #[no_main]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::run_tests)]
// Disable dynamic linking with the std library because there is no std library in our own os

use core::panic::PanicInfo;

use elysia_os::println;

// Disables name mangling so the linker can recognize the entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
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

test!("Basic VGA Print", || println!("Hello world!"));
test!("Long VGA Print", || println!(
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
));
test!("VGA Print new line", || println!("aaa\naaa\naaa"));
test!("Really long VGA Print", || {
    for _i in 0..=100 {
        println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    }
});
