use crate::{multitasking::memory::allocate_stack, userspace::elf_loader::Function};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Context {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
    rsp: u64, // stack pointer. aka the stack for the process
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

    pub fn as_ptr(self) -> *mut Self {
        &mut self.clone() as *mut Self
    }
}
