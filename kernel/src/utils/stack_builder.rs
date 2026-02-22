use x86_64::VirtAddr;

#[derive(Debug)]
pub struct StackBuilder {
    sp: VirtAddr,
    write_sp: *mut u64,
}

impl StackBuilder {
    pub fn new(sp: u64, write_sp: *mut u64) -> Self {
        Self {
            sp: VirtAddr::new(sp),
            write_sp,
        }
    }

    pub fn push(&mut self, value: u64) {
        unsafe { write_and_sub(&mut self.write_sp, value) };
        self.sp -= 8;
    }

    pub fn finish(self) -> VirtAddr {
        assert!(
            self.sp.is_aligned(16u64),
            "Stack pointer is not 16 byte aligned"
        );
        self.sp
    }
}

/// # Safety
/// Must provide valid pointer
unsafe fn write_and_sub(ptr: &mut *mut u64, data: u64) {
    unsafe {
        *ptr = ptr.sub(1);
        ptr.write(data);
    }
}
