use core::sync::atomic::AtomicU64;

use alloc::boxed::Box;
use x86_64::{
    VirtAddr,
    registers::model_specific::Msr,
    structures::paging::{Mapper, Page, Size4KiB, Translate},
};

use crate::{
    memory::page_table_wrapper::PageTableWrapped,
    multitasking::{
        context::Context,
        memory::{allocate_kernel_stack, allocate_stack},
        yielding::BlockType,
    },
    s_println,
    userspace::elf_loader::{Function, load_elf},
    utils::misc::write_and_sub,
};

#[derive(Debug)]
pub struct Process {
    pub pid: ProcessID,
    pub context: Context,
    pub state: State,
    pub page_table: PageTableWrapped,
    pub kernel_stack_top: VirtAddr,
}

// TODO: add threads, and make process just a wrapper/container of threads
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

        // TODO: Maybe let write_and_sub also take virt_stack_addr and sub it
        let (virt_stack_addr, mut virt_stack_write) = allocate_stack(16, &mut table.inner);

        init_stack_layout(&mut table, &mut virt_stack_write);

        let entry_point = load_elf(&mut table, program);
        let contxt = Context::new(
            entry_point as u64,
            &mut table,
            virt_stack_addr.as_u64() - 5 * 8,
        );
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

fn init_stack_layout(table: &mut PageTableWrapped, virt_stack_write: &mut *mut u64) {
    unsafe {
        write_and_sub(virt_stack_write, 0);
        write_and_sub(virt_stack_write, 0);
        write_and_sub(virt_stack_write, 0);
        write_and_sub(virt_stack_write, 0);
        write_and_sub(virt_stack_write, 1);
    };
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
