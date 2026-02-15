use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
        mapper::MapToError,
    },
};

use crate::{
    memory::paging::{FRAME_ALLOCATOR, MAPPER, get_l4_table},
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

    pub fn lock(&self) -> spin::MutexGuard<'_, A> {
        self.inner.lock()
    }
}

/// Copies the memory mapping of the kernel l4 table
/// into the table of something else (probably table for processes)
pub fn copy_kernel_mapping(table: &mut PageTable) {
    let kernel_l4 = get_l4_table(get_os().phys_mem_offset.unwrap());

    for i in 256..512 {
        table[i] = kernel_l4[i].clone();
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
