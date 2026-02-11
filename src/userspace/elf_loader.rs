use alloc::{
    collections::{btree_map::Range, vec_deque::VecDeque},
    vec::Vec,
};
use bootloader::bootinfo::FrameRange;
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Mapper, Page, PageTableFlags, PhysFrame, Size4KiB, Translate},
};
use xmas_elf::{ElfFile, program};

use crate::memory::paging::{FRAME_ALLOCATOR, MAPPER};

pub type Function = *const extern "C" fn() -> !;

// ELF is a file format that contains the actual code and instructions on which parts of the
// code need to be loaded where, and which parts of the file are instructions,
// which parts are memory, and which parts of the memory are read-only.
#[derive()]
pub struct ElfLoader {
    pub file: ElfFile<'static>,
}

impl ElfLoader {
    pub fn new(file: &'static [u8]) -> Self {
        let file = ElfFile::new(file).expect("Failed to parse elf file");

        Self { file: file }
    }

    pub fn load(&mut self) -> Function {
        // Loads all the segments that required loading into memory
        for segment in self
            .file
            .program_iter()
            .filter(|p| p.get_type().unwrap() == program::Type::Load)
        {
            let file_origin = MAPPER
                .try_get()
                .unwrap()
                .lock()
                .translate_addr(VirtAddr::from_ptr(self.file.input.as_ptr()));

            let virt_start = VirtAddr::new(segment.virtual_addr());
            let virt_end = virt_start + segment.mem_size();

            let phys_start = PhysAddr::new(segment.physical_addr());
            let phys_end = phys_start + segment.mem_size();

            let page_start: Page<Size4KiB> = Page::containing_address(virt_start);
            let page_end = Page::containing_address(virt_end - 1u64);

            let frame_start: PhysFrame<Size4KiB> = PhysFrame::containing_address(phys_start);
            let frame_end = PhysFrame::containing_address(phys_end - 1u64);

            for ele in Page::range_inclusive(page_start, page_end)
                .zip(PhysFrame::range_inclusive(frame_start, frame_end))
            {
                unsafe {
                    MAPPER
                        .try_get()
                        .unwrap()
                        .lock()
                        .map_to(
                            ele.0,
                            ele.1,
                            PageTableFlags::all(),
                            &mut *FRAME_ALLOCATOR.try_get().unwrap().lock(),
                        )
                        .unwrap()
                        .flush();
                }
            }
        }

        self.file.header.pt2.entry_point() as Function
    }
}
