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

        init_stack_layout(&mut table, &mut virt_stack_write, virt_stack_addr.as_u64());

        let entry_point = load_elf(&mut table, program);
        let contxt = Context::new(
            entry_point as u64,
            &mut table,
            virt_stack_addr.as_u64() - 15 * 8,
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

fn init_stack_layout(
    table: &mut PageTableWrapped,
    virt_stack_write: &mut *mut u64,
    virt_stack_addr: u64,
) {
    unsafe {
        // A. 先在栈的最顶端存入字符串 "init\0"
        // 字符串占用 5 字节，为了对齐我们按 8 字节处理
        let arg_str = "init\0";
        let str_len = arg_str.len();

        // 手动移动指针存入字符串
        *virt_stack_write = (virt_stack_write).sub(8);
        core::ptr::copy_nonoverlapping(arg_str.as_ptr(), *virt_stack_write as *mut u8, str_len);

        // B. 使用你的 write_and_sub 按照 ABI 逆序压栈

        // 1. 压入辅助向量结束符 AT_NULL (2个u64)
        write_and_sub(virt_stack_write, 0); // Value
        write_and_sub(virt_stack_write, 0); // Type: AT_NULL

        // 2. 压入环境变量结束符 envp[0] = NULL
        write_and_sub(virt_stack_write, 0);

        // 3. 压入 argv[1] = NULL (argv 结束符)
        write_and_sub(virt_stack_write, 0);

        // 4. 压入 argv[0] 指向我们刚才写的字符串地址
        write_and_sub(virt_stack_write, 12);

        // 5. 压入 argc = 1
        write_and_sub(virt_stack_write, 1);

        // 此时一共用了 6 个 u64，加上前面的字符串占用的 8 字节，
        // 最终 user_rsp 应该减去 (6 * 8 + 8) = 56 字节
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
