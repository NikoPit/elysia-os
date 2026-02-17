use x86_64::{
    VirtAddr,
    registers::control::{Cr3, Cr3Flags},
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
};

use crate::{
    memory::{paging::FRAME_ALLOCATOR, utils::copy_kernel_mapping},
    os::get_os,
};

pub struct PageTableWrapped {
    pub frame: PhysFrame<Size4KiB>,
    pub inner: OffsetPageTable<'static>,
}

impl Default for PageTableWrapped {
    fn default() -> Self {
        // allocates a frame for the l4 page table to be stored at
        let page_table_frame = FRAME_ALLOCATOR
            .get()
            .unwrap()
            .lock()
            .allocate_frame()
            .expect("No more space");

        let table_virt_addr = VirtAddr::new(
            page_table_frame.start_address().as_u64() + get_os().phys_mem_offset.unwrap().as_u64(),
        );

        // Get it as a page table
        let page_table = unsafe { &mut *(table_virt_addr.as_mut_ptr() as *mut PageTable) };

        page_table.zero();

        copy_kernel_mapping(page_table);

        Self {
            frame: page_table_frame,
            inner: unsafe { OffsetPageTable::new(page_table, get_os().phys_mem_offset.unwrap()) },
        }
    }
}
