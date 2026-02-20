/// Context for a CPU Core
#[derive(Debug)]
#[repr(C)]
pub struct CpuCoreContext {
    // Used on syscall_entry with swapgs
    pub gs_kernel_stack_top: u64,
    pub gs_user_stack_top: u64,
}
