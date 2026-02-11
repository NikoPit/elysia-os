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

use alloc::string::ToString;
use bootloader::{BootInfo, entry_point};
#[cfg(test)]
use elysia_os::debug_exit::debug_exit;
use elysia_os::driver::keyboard::scancode_processing::process_keypresses;
use elysia_os::filesystem::path::Path;
use elysia_os::filesystem::vfs::{FileData, VirtualFS};
use elysia_os::init;
use elysia_os::multitasking::kernel_task::executor::Executor;
use elysia_os::multitasking::kernel_task::task::Task;
use elysia_os::println;

entry_point!(k_main);

fn k_main(bootinfo: &'static BootInfo) -> ! {
    #[cfg(test)]
    debug_exit(elysia_os::debug_exit::QemuExitCode::Success);
    println!("Welcome to Elysia-OS v0.1.0");

    init(bootinfo);

    let mut executor = Executor::new();

    let a_txt = Path::new("/test/a.txt");
    VirtualFS.lock().create_file(a_txt.clone()).unwrap();
    VirtualFS
        .lock()
        .write_file(
            a_txt.clone(),
            FileData {
                content: "abc".to_string(),
            },
        )
        .unwrap();
    let content = VirtualFS.lock().read_file(a_txt.clone()).unwrap().content;

    println!("{:?}", content);

    // syscall test
    trigger_syscall();

    executor.spawn(Task::new(taskz()));
    executor.spawn(Task::new(process_keypresses()));
    executor.run();
}

async fn taskz() {
    println!("println from async task!");
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
    use elysia_os::panic_handler::handle_panic;

    handle_panic(_info);
}

fn trigger_syscall() {
    let syscall_number = 1; // write
    let fd = 1;
    let buf = b"Hello from syscall!\n".as_ptr();
    let count = 20;

    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") syscall_number,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            out("rcx") _, // 系统调用会破坏rcx和r11
            out("r11") _,
        );
    }
}
