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
use elysia_os::hardware_interrupt::HardwareInterrupt;
use elysia_os::init;
use elysia_os::os::get_os;
use elysia_os::print;
use elysia_os::s_print;
use elysia_os::s_println;
use elysia_os::testing;
use volatile::VolatilePtr;
use volatile::VolatileRef;
use x86_64::instructions::interrupts::int3;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
// Disable dynamic linking with the std library because there is no std library in our own os

use core::panic::PanicInfo;

use elysia_os::panic_handler;
use elysia_os::panic_handler::test_handle_panic;
use elysia_os::println;
use elysia_os::test;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt[HardwareInterrupt::Timer.as_u8()].set_handler_fn(timer_interrupt);

        idt
    };
}

extern "x86-interrupt" fn timer_interrupt(_stack_frame: InterruptStackFrame) {
    print!("a")
}
entry_point!(_start);
// Disables name mangling so the linker can recognize the entry point
fn _start(bootinfo: &'static BootInfo) -> ! {
    s_print!("\nVGA Printer deadlock ");
    init(bootinfo);

    IDT.load();

    for i in 0..=10000 {
        print!("-");
    }

    s_println!("[OK]\n");
    debug_exit(elysia_os::debug_exit::QemuExitCode::Success);

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_handle_panic(info);
}
