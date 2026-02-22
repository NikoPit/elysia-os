use elfloader::ElfBinary;
use x86_64::VirtAddr;

use crate::utils::aux::AuxType;

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

    pub fn push_aux_entries(&mut self, file: &ElfBinary) {
        self.push_aux_entry(AuxType::Null, 0);
        self.push_aux_entry(AuxType::EntryPointAddress, file.entry_point());
        self.push_aux_entry(
            AuxType::ProgramHeaderAmount,
            file.program_headers().count() as u64,
        );
        self.push_aux_entry(AuxType::ProgramHeaderNT, 5);
        self.push_aux_entry(AuxType::ProgramHeaderTable, 0);
        self.push_aux_entry(AuxType::PageSize, 4096);
    }

    fn push_aux_entry(&mut self, aux_type: AuxType, value: u64) {
        self.push(aux_type as u64);
        self.push(value);
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
