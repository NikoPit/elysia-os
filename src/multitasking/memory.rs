use core::sync::atomic::{AtomicU16, AtomicU64, Ordering};

use x86_64::{
    VirtAddr,
    structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB},
};

use crate::memory::paging::{FRAME_ALLOCATOR, MAPPER};

static AVALIBLE_MEMORY: AtomicU64 = AtomicU64::new(0x4444_4444_0000);

pub fn allocate_stack(pages: u64) -> VirtAddr {
    // skips the guard page
    let guard_page = allocate_page(pages);

    let start = guard_page + 1;
    for i in 0..pages {
        let page = start + i;

        unsafe {
            MAPPER.try_get().unwrap().lock().map_to(
                page,
                FRAME_ALLOCATOR
                    .try_get()
                    .unwrap()
                    .lock()
                    .allocate_frame()
                    .unwrap(),
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                &mut *FRAME_ALLOCATOR.try_get().unwrap().lock(),
            )
        };
    }

    // stack top
    (start + pages).start_address()
}

/// returns the start page
pub fn allocate_page(count: u64) -> Page<Size4KiB> {
    let page = Page::containing_address(VirtAddr::new(
        AVALIBLE_MEMORY.fetch_sub(4096, Ordering::Relaxed),
    ));

    for _ in 0..=count - 1 {
        AVALIBLE_MEMORY.fetch_sub(4096, Ordering::Relaxed);
    }

    page
}
