use futures_util::future::Select;

use crate::{
    memory::{page_table_wrapper::PageTableWrapped, paging::MAPPER},
    multitasking::{exit::exit_handler, memory::allocate_stack, process},
    os::get_os,
    println,
    userspace::elf_loader::Function,
};

// NOTE: the direction of the struct in memory and the stack is REVERSED
// therefore you need to push rbp - r15 and then rflags
// and also, ptr.sub(1) 6 times (rbp-r15) and then write the rflags
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Context {
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
        let (one, two) = allocate_stack(16, &mut table.inner);
        // stack top
        let mut phys_ptr: *mut u64 = two.as_mut_ptr();
        let virt_ptr: *mut u64 = one.as_mut_ptr();

        unsafe {
            // Push the exit handler, so when the entry point returns
            // switch() will call the exit handler
            phys_ptr = phys_ptr.sub(1);

            *phys_ptr = exit_handler as u64;

            // Give space for the entry point ptr
            phys_ptr = phys_ptr.sub(1);
            // Put the entry point pointer into the stack
            // to be used on switch()
            *phys_ptr = entry_point;

            // NOTE: the order which you write stuff into the stack
            // must be the EXACT SAME ORDER as its location
            // in the context struct AND the order which
            // you push/pop in the context switch

            // make space for the rbp - r15
            for _ in 0..6 {
                phys_ptr = phys_ptr.sub(1);
                phys_ptr.write(0);
            }

            phys_ptr = phys_ptr.sub(1);
            phys_ptr.write(0x202);
        }

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
            rsp: virt_ptr as u64,
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
