#![no_std]
// Disables main function to customize entry point
#![no_main]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
// renames main function for testing because we disabled main with #[no_main]
#![reexport_test_harness_main = "test_main"]
#![test_runner(testing::run_tests)]
use elysia_os::interrupts::init_idt;
use elysia_os::testing;
use x86_64::instructions::interrupts::int3;
// Disable dynamic linking with the std library because there is no std library in our own os

use core::panic::PanicInfo;

use elysia_os::panic_handler;
use elysia_os::panic_handler::test_handle_panic;
use elysia_os::println;
use elysia_os::test;

// Disables name mangling so the linker can recognize the entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init_idt();
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_handle_panic(info);
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
test!("Breakpoint interrupt crash", || int3());
