use x86_64::{VirtAddr, registers::control::Cr3Flags};

use crate::{
    gdt::GDT,
    memory::page_table_wrapper::PageTableWrapped,
    multitasking::{exit_handling::exit_handler, memory::allocate_stack},
    userspace::elf_loader::Function,
    utils::misc::{calc_cr3_value, write_and_sub},
};

// NOTE: the direction of the struct in memory and the stack is REVERSED
// therefore you need to push rbp - r15 and then rflags
// and also, ptr.sub(1) 6 times (rbp-r15) and then write the rflags
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Context {
    cr3: u64, // +0
    rsp: u64, // +8
    r15: u64, // +16
    r14: u64, // +24
    r13: u64, // +32
    r12: u64, // +40
    rbx: u64, // +48
    rbp: u64, // +56
}

impl Context {
    fn new(entry_point: u64, table: &mut PageTableWrapped, context_type: ContextType) -> Self {
        let (virt_stack_addr, write_addr) = allocate_stack(16, &mut table.inner);
        let mut write_ptr: *mut u64 = (write_addr).as_mut_ptr();

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
            cr3: calc_cr3_value(table.frame.start_address(), Cr3Flags::empty()),
            rsp: init_memory(&mut write_ptr, entry_point, virt_stack_addr, context_type),
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            rbx: 0,
            rbp: 0,
        }
    }

    pub fn kernel(entry_point: u64, table: &mut PageTableWrapped) -> Self {
        Self::new(entry_point, table, ContextType::Kernel)
    }

    pub fn user(entry_point: u64, table: &mut PageTableWrapped) -> Self {
        Self::new(entry_point, table, ContextType::User)
    }

    pub fn as_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }
}

fn init_memory(
    write_ptr: &mut *mut u64,
    entry_point: u64,
    virt_stack_addr: VirtAddr,
    context_type: ContextType,
) -> u64 {
    match context_type {
        ContextType::Kernel => init_memory_kernel(write_ptr, entry_point, virt_stack_addr),
        ContextType::User => init_memory_user(write_ptr, entry_point, virt_stack_addr),
    }
}

/// Initalizes the memory of the context struct
/// Returns the virtual address of the stack top
fn init_memory_kernel(
    write_ptr: &mut *mut u64,
    entry_point: u64,
    virt_stack_addr: VirtAddr,
) -> u64 {
    unsafe {
        write_and_sub(write_ptr, exit_handler as *mut Function as u64);
        write_and_sub(write_ptr, entry_point);

        // NOTE: the order which you write stuff into the stack
        // must be the EXACT SAME ORDER as its location
        // in the the order which
        // you push/pop in the context switch

        // rflags
        write_and_sub(write_ptr, 0x202);
    }

    virt_stack_addr.as_u64() - 3 * 8
}

fn init_memory_user(write_ptr: &mut *mut u64, entry_point: u64, virt_stack_addr: VirtAddr) -> u64 {
    unsafe {
        write_and_sub(write_ptr, GDT.1.user_data.0 as u64); // SS
        write_and_sub(write_ptr, virt_stack_addr.as_u64() - 5 * 8); // RSP
        write_and_sub(write_ptr, 0x202); // RFlags
        write_and_sub(write_ptr, GDT.1.user_code.0 as u64);
        write_and_sub(write_ptr, entry_point);
    }

    virt_stack_addr.as_u64() - 5 * 8
}

pub enum ContextType {
    Kernel,
    User,
}
