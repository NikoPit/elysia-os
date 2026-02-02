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
use elysia_os::misc::hlt_loop;
use elysia_os::os::get_os;
use elysia_os::print;
use elysia_os::s_print;
use elysia_os::s_println;
use elysia_os::testing;
use x86_64::VirtAddr;
use x86_64::instructions::interrupts::int3;
use x86_64::structures::paging::Page;
// Disable dynamic linking with the std library because there is no std library in our own os

use core::panic::PanicInfo;

use elysia_os::panic_handler;
use elysia_os::panic_handler::test_handle_panic;
use elysia_os::println;
use elysia_os::test;
entry_point!(_start);
fn _start(bootinfo: &'static BootInfo) -> ! {
    s_println!("\nMemory Mapping");

    get_os().init(bootinfo);

    let page = Page::containing_address(VirtAddr::new(0x153212562324));

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

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME: 这并不安全，我们这样做只是为了测试。
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}
