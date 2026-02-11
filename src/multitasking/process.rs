use core::sync::atomic::AtomicU64;

use x86_64::{
    VirtAddr,
    structures::paging::{OffsetPageTable, PageTable},
};

use crate::{multitasking::context::Context, userspace::elf_loader::Function};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Process {
    pub pid: ProcessID,
    pub context: Context,
}

impl Process {
    pub fn new(entry_point: Function) -> Self {
        Self {
            pid: ProcessID::new(),
            context: Context::new(entry_point as u64),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ProcessID(u64);

impl ProcessID {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        Self(NEXT_ID.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
    }
}
