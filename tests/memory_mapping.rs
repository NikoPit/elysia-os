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
use elysia_os::paging::BootinfoFrameAllocator;
use elysia_os::paging::init_mapper;
use elysia_os::print;
use elysia_os::s_print;
use elysia_os::s_println;
use elysia_os::testing;
use x86_64::PhysAddr;
use x86_64::VirtAddr;
use x86_64::instructions::interrupts::int3;
use x86_64::structures::paging::FrameAllocator;
use x86_64::structures::paging::Mapper;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::structures::paging::Page;
use x86_64::structures::paging::PhysFrame;
use x86_64::structures::paging::Size4KiB;
use x86_64::structures::paging::Translate;
// Disable dynamic linking with the std library because there is no std library in our own os

use core::panic::PanicInfo;

use elysia_os::panic_handler;
use elysia_os::panic_handler::test_handle_panic;
use elysia_os::println;
use elysia_os::test;

const RANDOM_ADDR: u64 = 0x153212562324;

entry_point!(_start);
fn _start(bootinfo: &'static BootInfo) -> ! {
    s_print!("\nMemory Mapping ");

    let page = Page::containing_address(VirtAddr::new(RANDOM_ADDR));
    let mut frame_allocator: BootinfoFrameAllocator =
        unsafe { BootinfoFrameAllocator::new(&bootinfo.memory_map) };
    let mut mapper = init_mapper(bootinfo);
    get_os().init(bootinfo, &mut mapper, &mut frame_allocator);

    create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // 通过新的映射将字符串 `New!`  写到屏幕上。
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // TODO check the output to see if the "NEW!" string have been printed
    s_println!("[OK]\n\n\n\n\n");
    s_println!("--------------------------------------------------------");
    s_println!("not fully implemented");

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
