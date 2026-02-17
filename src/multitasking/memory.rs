use core::sync::atomic::{AtomicU16, AtomicU64, Ordering};

use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
        Size4KiB,
    },
};

use crate::{
    memory::paging::{FRAME_ALLOCATOR, MAPPER},
    os::get_os,
    println, s_print,
};

static AVALIBLE_MEMORY: AtomicU64 = AtomicU64::new(0x4444_0000);
pub const USER_STACK: u64 = 0x7000_0000_0000;

/// Returns the virtual address of the stack top
/// and the offsetted physical address of the stack top
pub fn allocate_stack(pages: u64, table: &mut OffsetPageTable<'static>) -> (VirtAddr, VirtAddr) {
    // skips the guard page
    let guard_page = allocate_page(pages);
    let mut frame_allocator = FRAME_ALLOCATOR.try_get().unwrap().lock();
    let mut last_frame: Option<PhysFrame> = None;

    let start = guard_page + 1;
    for i in 0..pages {
        let page = start + i;
        let frame = frame_allocator.allocate_frame().unwrap();

        unsafe {
            table
                .map_to(
                    page,
                    frame,
                    PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                    &mut *frame_allocator,
                )
                .unwrap()
                .flush();
        };

        last_frame = Some(frame);
    }

    // stack top
    (
        (start + pages).start_address(),
        VirtAddr::new(
            last_frame.unwrap().start_address().as_u64()
                + get_os().phys_mem_offset.unwrap().as_u64()
                + 4096,
        ),
    )
}

/// returns the start page
pub fn allocate_page(count: u64) -> Page<Size4KiB> {
    // NOTE: the start page is the page at the TOP not the BOTTOM
    // page. because stack goes from UP to DOWN!!!!!!
    Page::containing_address(VirtAddr::new(
        USER_STACK, //AVALIBLE_MEMORY.fetch_add((count + 1) * 4096, Ordering::Relaxed),
    ))
}
