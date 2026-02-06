use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        self, Mapper, Page, PageTableFlags, PhysFrame, Size4KiB, mapper::MapToError,
    },
};

use crate::{
    memory::{
        paging::{FRAME_ALLOCATOR, MAPPER},
        utils::get_map_location,
    },
    os::get_os,
};

pub fn map_phys_size(start: u64, size: u64) -> Result<(), MapToError<Size4KiB>> {
    map_phys_area(start, start + size)
}

pub fn map_phys_area(start: u64, end: u64) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(get_map_location(start));
        let heap_end = VirtAddr::new(get_map_location(end));
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    let frame_range = {
        let frame_start = PhysAddr::new(start);
        let frame_end = PhysAddr::new(end);
        let frame_start = PhysFrame::containing_address(frame_start);
        let frame_end = PhysFrame::containing_address(frame_end);
        PhysFrame::range_inclusive(frame_start, frame_end)
    };

    // map the pages of heap memory to virtual memory
    for i in 0..=frame_range.size() {
        let frame = frame_range.into_iter().nth(i as usize);
        let page = page_range.into_iter().nth(i as usize);
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            MAPPER
                .get()
                .unwrap()
                .lock()
                .map_to(
                    page.unwrap(),
                    frame.unwrap(),
                    flags,
                    &mut *FRAME_ALLOCATOR.get().unwrap().lock(),
                )?
                .flush()
        };
    }

    Ok(())
}
