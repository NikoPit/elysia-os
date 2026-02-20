use alloc::sync::Arc;
use bootloader::BootInfo;
use conquer_once::spin::OnceCell;
use spin::Mutex;
use x86_64::VirtAddr;

use crate::{
    memory::{
        heap::init_heap,
        paging::{BootinfoFrameAllocator, FRAME_ALLOCATOR, MAPPER, init_mapper},
    },
    os::get_os,
};

pub mod fixed_block_size;
pub mod heap;
pub mod page_table_wrapper;
pub mod paging;
pub mod utils;

pub static PHYSICAL_MEMORY_OFFSET: OnceCell<u64> = OnceCell::uninit();

pub fn init(bootinfo: &'static BootInfo) {
    let mut mapper = init_mapper(bootinfo);
    let mut frame_allocator = unsafe { BootinfoFrameAllocator::new(&bootinfo.memory_map) };
    init_heap(&mut mapper, &mut frame_allocator).expect("Failed heap initilization");

    // [TODO] maybe i should move some stuff out of the os struct? tho if it works, dont touch it
    let mapper = Arc::new(Mutex::new(mapper));
    let frame_allocator = Arc::new(Mutex::new(frame_allocator));

    // inits mapper and frame allocator
    MAPPER.get_or_init(|| mapper.clone());
    FRAME_ALLOCATOR.get_or_init(|| frame_allocator.clone());
    PHYSICAL_MEMORY_OFFSET.get_or_init(|| bootinfo.physical_memory_offset);
}
