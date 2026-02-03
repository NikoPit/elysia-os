#![no_std]
// Disables main function to customize entry point
#![no_main]
#![feature(abi_x86_interrupt, custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(elysia_os::testing::run_tests)]
// renames main function for testing because we disabled main with #[no_main]
// Disable dynamic linking with the std library because there is no std library in our own os
//
extern crate alloc;

use core::panic::PanicInfo;

use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};
use elysia_os::misc::hlt_loop;
use elysia_os::multitasking::executor::Executor;
use elysia_os::multitasking::task::Task;
use elysia_os::paging::{BootinfoFrameAllocator, init_mapper};
use elysia_os::panic_handler::handle_panic;
use elysia_os::{os::get_os, println};
use x86_64::VirtAddr;
use x86_64::structures::paging::{FrameAllocator, Page, Size4KiB, Translate, frame};

entry_point!(k_main);

fn k_main(bootinfo: &'static BootInfo) -> ! {
    println!("Welcome to Elysia-OS v0.1.0");

    let mut frame_allocator: BootinfoFrameAllocator =
        unsafe { BootinfoFrameAllocator::new(&bootinfo.memory_map) };
    let mut mapper = init_mapper(bootinfo);
    let mut executor = Executor::new();

    get_os().init(bootinfo, &mut mapper, &mut frame_allocator);

    executor.spawn(Task::new(taskz()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("ts worked");

    hlt_loop();
}

async fn taskz() {
    println!("println from async task!");
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
