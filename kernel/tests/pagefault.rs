#![no_std]
// Disables main function to customize entry point
#![no_main]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
// renames main function for testing because we disabled main with #[no_main]
#![reexport_test_harness_main = "test_main"]
#![test_runner(testing::run_tests)]
#![allow(dead_code)]
use bootloader::BootInfo;
use bootloader::entry_point;
use elysia_os::debug_exit::debug_exit;
use elysia_os::misc::hlt_loop;
use elysia_os::s_println;
use elysia_os::testing;
// Disable dynamic linking with the std library because there is no std library in our own os

use core::panic::PanicInfo;

use elysia_os::panic_handler::test_handle_panic;

entry_point!(_start);
fn _start(_bootinfo: &'static BootInfo) -> ! {
    // get_os().init();
    //s_print!("\nPagefault handling ");

    //pagefault();

    //s_print!("[OK]\n\n");
    //debug_exit(elysia_os::debug_exit::QemuExitCode::Success);

    // TODO implement this. note for future me:
    // the reason that i didnt is becuase when the pagefault was successfully handled
    // the system will just halt, and will not exit

    s_println!(
        "------------------------------------------------------------------------------------------"
    );
    s_println!("                           not yet implemented");
    s_println!(
        "------------------------------------------------------------------------------------------"
    );

    debug_exit(elysia_os::debug_exit::QemuExitCode::Success);

    hlt_loop();
}

fn pagefault() {
    unsafe {
        *(0xdeadbeef as *mut u8) = 114;
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_handle_panic(info);
}
