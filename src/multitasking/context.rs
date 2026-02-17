use futures_util::future::Select;
use x86_64::{VirtAddr, registers::control::Cr3Flags};

use crate::{
    memory::{page_table_wrapper::PageTableWrapped, paging::MAPPER},
    multitasking::{exit::exit_handler, memory::allocate_stack, process},
    os::get_os,
    println, s_print, s_println,
    userspace::elf_loader::Function,
};

// NOTE: the direction of the struct in memory and the stack is REVERSED
// therefore you need to push rbp - r15 and then rflags
// and also, ptr.sub(1) 6 times (rbp-r15) and then write the rflags
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Context {
    cr3: u64,
    rsp: u64, // stack pointer. aka the stack for the process
    rflags: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

impl Context {
    // TODO: fix ts (seprate pagetable for each process)
    // im gonna moveon and fix the pagefault shit first
    pub fn new(entry_point: u64, table: &mut PageTableWrapped) -> Self {
        let (virt_stack_addr, write_addr) = allocate_stack(16, &mut table.inner);
        let mut write_ptr: *mut u64 = (write_addr).as_mut_ptr();

        let addr = table.frame.start_address();
        let value = ((false as u64) << 63) | addr.as_u64() | Cr3Flags::empty().bits() as u64;

        unsafe {
            // Push the exit handler, so when the entry point returns
            // switch() will call the exit handler
            write_ptr = write_ptr.sub(1);
            *write_ptr = exit_handler as u64;

            // Give space for the entry point ptr
            write_ptr = write_ptr.sub(1);
            // Put the entry point pointer into the stack
            // to be used on switch()
            *write_ptr = entry_point;

            // NOTE: the order which you write stuff into the stack
            // must be the EXACT SAME ORDER as its location
            // in the context struct AND the order which
            // you push/pop in the context switch

            // rbp - r15
            for _ in 0..6 {
                write_ptr = write_ptr.sub(1);
                write_ptr.write(0);
            }

            // rflags
            write_ptr = write_ptr.sub(1);
            write_ptr.write(0x202);

            // cr3
            write_ptr = write_ptr.sub(1);
            write_ptr.write(value);
        }

        let virt_stack_addr = virt_stack_addr.as_u64() - 9 * 8;

        let val = unsafe { *write_ptr };
        s_println!(
            "DEBUG: Physical write check (at {:p}): {:#x}",
            write_ptr,
            val
        );

        // 验证虚拟地址映射
        // 这里的 table 是你刚才映射过的那个 table
        use x86_64::structures::paging::Translate;
        let phys = table
            .inner
            .translate_addr(VirtAddr::new(virt_stack_addr))
            .unwrap();
        s_println!(
            "DEBUG: Virtual RSP {:#x} maps to Physical {:?}",
            virt_stack_addr,
            phys
        );

        // Now the rsp is pointing at the stack top
        // under the entry point and all the other bs (r15-rbp)
        // so it wont accidently read / write into them

        // The stack:
        // GUARD PAGE
        // EXIT HANDLER
        // ENTRY POINT
        // r15-rbp
        // EMPTY SPACE <-
        Self {
            // TODO: use virtual address
            cr3: value,
            rsp: virt_stack_addr as u64,
            rflags: 0x202,
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            rbx: 0,
            rbp: 0,
        }
    }
    pub fn empty() -> Self {
        Self {
            cr3: 0,
            rflags: 0,
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            rbx: 0,
            rbp: 0,
            rsp: 0,
        }
    }

    pub fn as_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }
}
