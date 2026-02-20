use x86_64::{
    PhysAddr, VirtAddr,
    registers::control::{Cr3, Cr3Flags},
};

/// # Safety
/// Must provide valid pointer
pub unsafe fn write_and_sub(ptr: &mut *mut u64, data: u64) {
    unsafe {
        *ptr = ptr.sub(1);
        ptr.write(data);
    }
}

pub fn calc_cr3_value(addr: PhysAddr, flags: Cr3Flags) -> u64 {
    ((false as u64) << 63) | addr.as_u64() | flags.bits()
}
