use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, mapper::MapToError,
    },
};

use crate::{
    memory::paging::{FRAME_ALLOCATOR, MAPPER},
    os::get_os,
};

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

pub fn map_size(start: u64, size: u64) -> Result<(), MapToError<Size4KiB>> {
    map_area(start, start + size - 1u64)
}

pub fn map_area(start: u64, end: u64) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(start);
        let heap_end = VirtAddr::new(end);
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // map the pages of heap memory to virtual memory
    for page in page_range {
        let frame = FRAME_ALLOCATOR
            .get()
            .unwrap()
            .lock()
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            MAPPER
                .get()
                .unwrap()
                .lock()
                .map_to(
                    page,
                    frame,
                    flags,
                    &mut *FRAME_ALLOCATOR.get().unwrap().lock(),
                )?
                .flush()
        };
    }

    Ok(())
}

pub fn get_offsetted_location(phys: u64) -> u64 {
    phys + get_os().phys_mem_offset.unwrap().as_u64()
}
