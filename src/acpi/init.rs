use acpi::{AcpiTable, AcpiTables, rsdp::Rsdp};
use x86_64::structures::paging::{FrameAllocator, Mapper, Size4KiB};

use crate::{
    acpi::handler::ACPIHandler,
    paging::{FRAME_ALLOCATOR, MAPPER},
    systemcall::implementations,
};

pub fn init_acpi() -> AcpiTables<ACPIHandler> {
    let handler = ACPIHandler::new(
        MAPPER.try_get().unwrap().lock(),
        FRAME_ALLOCATOR.try_get().unwrap().lock(),
    );
    let rsdp = unsafe { Rsdp::search_for_on_bios(handler) }.expect("Failed searching for Rsdp");
    unsafe {
        AcpiTables::from_rsdp(handler, rsdp.physical_start)
            .expect("Failed to get ACPI Table from rsdp")
    }
}
