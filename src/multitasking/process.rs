use core::sync::atomic::AtomicU64;

use x86_64::{
    VirtAddr,
    structures::paging::{Mapper, Page, Size4KiB, Translate},
};

use crate::{
    memory::page_table_wrapper::PageTableWrapped,
    multitasking::{context::Context, memory::allocate_kernel_stack, yielding::BlockType},
    s_println,
    userspace::elf_loader::{Function, load_elf},
};

#[derive(Debug)]
pub struct Process {
    pub pid: ProcessID,
    pub context: Context,
    pub state: State,
    pub page_table: PageTableWrapped,
    pub kernel_stack_top: VirtAddr,
}

impl Default for Process {
    fn default() -> Self {
        Self {
            page_table: PageTableWrapped::default(),
            pid: ProcessID::default(),
            context: Context::default(),
            state: State::Ready,
            kernel_stack_top: VirtAddr::zero(),
        }
    }
}

impl Process {
    pub fn new(program: &[u8]) -> Self {
        let mut table = PageTableWrapped::default();
        let entry_point = load_elf(&mut table, program);
        let result = table.inner.translate(VirtAddr::new(0x4000_0000_0000));
        s_println!("{:?}", result);
        let contxt = Context::user(entry_point as u64, &mut table);
        let kernel_stack_top = allocate_kernel_stack(16, &mut table.inner);

        Self {
            page_table: table,
            pid: ProcessID::default(),
            context: contxt,
            state: State::Ready,
            kernel_stack_top,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ProcessID(pub u64);

impl Default for ProcessID {
    fn default() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        Self(NEXT_ID.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum State {
    Ready, // ready to run (in a queue)
    Running,
    Blocked(BlockType), // stuck, waiting for something (like keyboard input)
    Zombie,             // Exited process
}
