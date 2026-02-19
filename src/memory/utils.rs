use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
        Size4KiB, mapper::MapToError, page_table::PageTableEntry,
    },
};

use crate::{
    memory::paging::{FRAME_ALLOCATOR, MAPPER, get_l4_table},
    multitasking::memory::USER_STACK_BOTTOM,
    os::get_os,
    println, s_println,
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
    let l4_binding = MAPPER.get().unwrap().lock();
    let kernel_l4 = l4_binding.level_4_table();
    //s_println!("{:#?}", kernel_l4[0]);

    for i in 0..512 {
        table[i] = kernel_l4[i].clone();
    }

    // TODO: idk if this is a good idea
    let stack_p4_index = VirtAddr::new(USER_STACK_BOTTOM).p4_index();
    table[usize::from(stack_p4_index)] = PageTableEntry::new(); // 清空这一项
}

pub fn apply_offset(num: u64) -> u64 {
    num + get_os().phys_mem_offset.unwrap().as_u64()
}

pub fn map_size(
    mapper: &mut OffsetPageTable<'static>,
    start: u64,
    size: u64,
    flags: PageTableFlags,
) -> Result<(), MapToError<Size4KiB>> {
    map_area(mapper, start, start + size - 1u64, flags)
}

pub fn map_area(
    mapper: &mut OffsetPageTable,
    start: u64,
    end: u64,
    flags: PageTableFlags,
) -> Result<(), MapToError<Size4KiB>> {
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
        unsafe {
            mapper
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
