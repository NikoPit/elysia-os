use core::panic;

use acpi::{AcpiError, AcpiTable, AcpiTables, rsdp::Rsdp};
use alloc::sync::Arc;
use spin::Mutex;
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Size4KiB};

use crate::{
    acpi::{ACPI_TABLE, handler::ACPIHandler},
    memory::paging::BootinfoFrameAllocator,
    println,
    systemcall::{error, implementations},
};

pub fn init_acpi() {
    let handler = ACPIHandler {};
    // [TODO] i dont think its safe to assume everything is on BIOS
    let rsdp = unsafe { Rsdp::search_for_on_bios(handler).expect("Failed to search RSDP") };

    ACPI_TABLE.try_get_or_init(|| unsafe {
        AcpiTables::from_rsdt(handler, 2, rsdp.rsdt_address() as usize)
            .expect("Failed to parse ACPI Table from RSDT")
    });
}
