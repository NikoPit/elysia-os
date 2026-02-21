use core::sync::atomic::AtomicU64;

use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTableFlags, PhysFrame, Size4KiB,
    },
};

use crate::{
    memory::{paging::FRAME_ALLOCATOR, utils::apply_offset},
    s_println,
};

static USER_STACK: AtomicU64 = AtomicU64::new(0x3000_0000);
static KERNEL_STACK: AtomicU64 = AtomicU64::new(0xFFFF_8000_1000_0000);

/// Returns the virtual address of the stack top
/// and the offsetted physical address of the stack top
///
/// Note: The phys addr of the stack top is the addr of the
/// last frame, so if you writes more then 4KiB of memory
/// it will cause undefined behaviour
pub fn allocate_stack(pages: u64, table: &mut OffsetPageTable<'static>) -> (VirtAddr, *mut u64) {
    // skips the guard page
    let guard_page = allocate_user_page(pages);

    let mut frame_allocator = FRAME_ALLOCATOR.try_get().unwrap().lock();

    let mut last_frame = None;
    let start = guard_page + 1;
    for i in 0..pages {
        let page = start + i;
        let frame = frame_allocator.allocate_frame().expect("Memory full.");

        unsafe {
            table
                .map_to(
                    page,
                    frame,
                    PageTableFlags::PRESENT
                        | PageTableFlags::WRITABLE
                        | PageTableFlags::USER_ACCESSIBLE,
                    &mut *frame_allocator,
                )
                .unwrap()
                .flush();
        };

        last_frame = Some(frame);
    }

    // Stack top
    (
        (start + pages).start_address(),
        (apply_offset(last_frame.unwrap().start_address().as_u64() + 4096) as *mut u64),
    )
}

pub fn allocate_kernel_stack(pages: u64, table: &mut OffsetPageTable<'static>) -> VirtAddr {
    let guard_page = allocate_kernel_page(pages);
    let start = guard_page + 1;
    let mut frame_allocator = FRAME_ALLOCATOR.try_get().unwrap().lock();

    for i in 0..pages {
        let page = start + i;
        let frame = frame_allocator.allocate_frame().expect("Memory full.");

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
    }

    (start + pages).start_address()
}

fn allocate_user_page(count: u64) -> Page<Size4KiB> {
    Page::containing_address(VirtAddr::new(
        USER_STACK.fetch_add((count + 1) * 4096, core::sync::atomic::Ordering::Relaxed),
    ))
}
fn allocate_kernel_page(count: u64) -> Page<Size4KiB> {
    Page::containing_address(VirtAddr::new(
        KERNEL_STACK.fetch_add((count + 1) * 4096, core::sync::atomic::Ordering::Relaxed),
    ))
}
