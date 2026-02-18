use core::sync::atomic::AtomicU64;

use crate::{
    memory::page_table_wrapper::PageTableWrapped,
    multitasking::{context::Context, yielding::BlockType},
    userspace::elf_loader::Function,
};

#[derive(Debug)]
pub struct Process {
    pub pid: ProcessID,
    pub context: Context,
    pub state: State,
    pub page_table: PageTableWrapped,
}

impl Default for Process {
    fn default() -> Self {
        Self {
            page_table: PageTableWrapped::default(),
            pid: ProcessID::default(),
            context: Context::default(),
            state: State::Ready,
        }
    }
}

impl Process {
    pub fn new(entry_point: Function) -> Self {
        let mut table = PageTableWrapped::default();
        let contxt = Context::kernel(entry_point as u64, &mut table);
        Self {
            page_table: table,
            pid: ProcessID::default(),
            context: contxt,
            state: State::Ready,
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
