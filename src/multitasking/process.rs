use core::sync::atomic::AtomicU64;

use x86_64::{
    VirtAddr,
    structures::paging::{OffsetPageTable, PageTable},
};

use crate::{
    multitasking::{blocked::BlockType, context::Context},
    println,
    userspace::elf_loader::Function,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Process {
    pub pid: ProcessID,
    pub context: Context,
    pub state: State,
}

impl Process {
    pub fn default() -> Self {
        Self {
            pid: ProcessID::new(),
            context: Context::empty(),
            state: State::Ready,
        }
    }

    pub fn new(entry_point: Function) -> Self {
        Self {
            pid: ProcessID::new(),
            context: Context::new(entry_point as u64),
            state: State::Ready,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ProcessID(pub u64);

impl ProcessID {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        Self(NEXT_ID.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum State {
    Ready, // ready to run (in a queue)
    Running,
    Blocked(BlockType), // stuck, waiting for something (like keyboard input)
    Zombie,
}
