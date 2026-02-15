use futures_util::future::Select;

use crate::{
    multitasking::{exit::exit_handler, memory::allocate_stack, process},
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
    pub fn new(entry_point: u64) -> Self {
        // stack top
        let mut ptr: *mut u64 = allocate_stack(16).as_mut_ptr();

        unsafe {
            // Push the exit handler, so when the entry point returns
            // switch() will call the exit handler
            ptr = ptr.sub(1);

            *ptr = exit_handler as u64;

            // Give space for the entry point ptr
            ptr = ptr.sub(1);
            // Put the entry point pointer into the stack
            // to be used on switch()
            *ptr = entry_point;

            // NOTE: the order which you write stuff into the stack
            // must be the EXACT SAME ORDER as its location
            // in the context struct AND the order which
            // you push/pop in the context switch

            // make space for the rbp - r15
            for _ in 0..6 {
                ptr = ptr.sub(1);
                ptr.write(0);
            }

            ptr = ptr.sub(1);
            ptr.write(0x202);
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
            rsp: ptr as u64,
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
