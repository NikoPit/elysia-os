use core::sync::atomic::AtomicU64;

use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTableFlags, PhysFrame, Size4KiB,
    },
};

use crate::{
    memory::{
        manager::{allocate_kernel_mem, allocate_user_mem},
        paging::FRAME_ALLOCATOR,
        utils::apply_offset,
    },
    s_println,
};

/// Returns the virtual address of the stack top
/// and the offsetted physical address of the stack top
///
/// Note: The phys addr of the stack top is the addr of the
/// last frame, so if you writes more then 4KiB of memory
/// it will cause undefined behaviour
pub fn allocate_stack(pages: u64, table: &mut OffsetPageTable<'static>) -> (VirtAddr, *mut u64) {
    let (_, end_addr, write_ptr) = allocate_user_mem(
        pages,
        table,
        PageTableFlags::USER_ACCESSIBLE | PageTableFlags::WRITABLE | PageTableFlags::PRESENT,
    );

    (end_addr, write_ptr)
}

pub fn allocate_kernel_stack(pages: u64, table: &mut OffsetPageTable<'static>) -> VirtAddr {
    allocate_kernel_mem(
        pages,
        table,
        PageTableFlags::WRITABLE | PageTableFlags::PRESENT,
    )
    .1
}
