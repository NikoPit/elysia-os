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

#[cfg(test)]
use core::iter::Successors;
use core::panic::PanicInfo;

use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::sync::Arc;
use bootloader::{BootInfo, entry_point};
use conquer_once::spin::OnceCell;
use elysia_os::acpi::init::init_acpi;
#[cfg(test)]
use elysia_os::debug_exit::debug_exit;
use elysia_os::driver::keyboard::scancode_processing::process_keypresses;
use elysia_os::memory::heap::init_heap;
use elysia_os::memory::paging::{BootinfoFrameAllocator, init_mapper};
use elysia_os::misc::hlt_loop;
use elysia_os::multitasking::executor::Executor;
use elysia_os::multitasking::task::Task;
use elysia_os::panic_handler::handle_panic;
use elysia_os::{os::get_os, println};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::VirtAddr;
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, Size4KiB, Translate, frame,
};

entry_point!(k_main);

fn k_main(bootinfo: &'static BootInfo) -> ! {
    #[cfg(test)]
    debug_exit(elysia_os::debug_exit::QemuExitCode::Success);
    println!("Welcome to Elysia-OS v0.1.0");

    let mut mapper = init_mapper(bootinfo);
    let mut frame_allocator = unsafe { BootinfoFrameAllocator::new(&bootinfo.memory_map) };

    init_heap(&mut mapper, &mut frame_allocator).expect("Failed heap initilization");

    let mut mapper = Arc::new(Mutex::new(mapper));
    let mut frame_allocator = unsafe { Arc::new(Mutex::new(frame_allocator)) };
    get_os().init(bootinfo, mapper.clone(), frame_allocator.clone());

    let table = init_acpi(mapper.clone(), frame_allocator.clone());

    let mut executor = Executor::new();

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
