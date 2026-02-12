use futures_util::future::Select;

use crate::{
    multitasking::{memory::allocate_stack, process},
    println,
    userspace::elf_loader::Function,
};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Context {
    rsp: u64, // stack pointer. aka the stack for the process
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

impl Context {
    pub fn new(entry_point: u64) -> Self {
        let mut ptr: *mut u64 = allocate_stack(16).as_mut_ptr();

        unsafe {
            // Give space for the entry point ptr
            ptr = ptr.sub(1);
            // Put the entry point pointer into the stack
            // to be used on switch()
            *ptr = entry_point;

            for _ in 0..6 {
                ptr = ptr.sub(1);
                ptr.write(0);
            }
        }

        Self {
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            rbx: 0,
            rbp: 0,
            rsp: ptr as u64,
        }
    }
    pub fn empty() -> Self {
        Self {
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
