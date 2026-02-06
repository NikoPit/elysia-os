use alloc::sync::Arc;
use conquer_once::spin::OnceCell;
use linked_list_allocator::LockedHeap;
use spin::Mutex;
use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, mapper::MapToError,
    },
};

use crate::heap_allocators::{Locked, fixed_block_size::FixedBlockSizeAllocator};

#[global_allocator]
static HEAP_ALLOCATOR: Locked<FixedBlockSizeAllocator> =
    Locked::new(FixedBlockSizeAllocator::new());

// Memory area for the heap
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

// Map the memory area for the heap from physical memory to virt memory
// and do some other stuff
pub fn init_heap(
    mapper: Arc<Mutex<impl Mapper<Size4KiB>>>,
    frame_allocator: Arc<Mutex<impl FrameAllocator<Size4KiB>>>,
) -> Result<(), MapToError<Size4KiB>> {
    // Page range of the heap
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE as u64 - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // map the pages of heap memory to virtual memory
    for page in page_range {
        let frame = frame_allocator
            .lock()
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper
                .lock()
                .map_to(page, frame, flags, &mut *frame_allocator.lock())?
                .flush()
        };
    }

    // initalize the heap allocator with the heap memory
    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}
