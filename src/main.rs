#![no_std]
// Disables main function to customize entry point
#![no_main]
#![feature(abi_x86_interrupt)]
// renames main function for testing because we disabled main with #[no_main]
// Disable dynamic linking with the std library because there is no std library in our own os

use elysia_os::{
    os::get_os,
    println,
};

use x86_64::instructions::interrupts::int3;

// Disables name mangling so the linker can recognize the entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Elysia-OS v0.1.0");

    get_os().init();
    int3();

    loop {}
}
