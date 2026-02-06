use acpi::{AcpiTable, AcpiTables, rsdp::Rsdp};
use alloc::sync::Arc;
use spin::Mutex;
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Size4KiB};

use crate::{
    acpi::handler::ACPIHandler, paging::BootinfoFrameAllocator, println,
    systemcall::implementations,
};

pub fn init_acpi(
    mapper: Arc<Mutex<OffsetPageTable<'static>>>,
    frame_allocator: Arc<Mutex<BootinfoFrameAllocator>>,
) -> AcpiTables<ACPIHandler> {
    println!("init acpi start");
    let handler = ACPIHandler::new(mapper.clone(), frame_allocator.clone());
    println!("step2");
    let rsdp = unsafe { Rsdp::search_for_on_bios(handler) }.expect("Failed searching for Rsdp");
    println!("step3");
    unsafe {
        AcpiTables::from_rsdp(handler, rsdp.physical_start)
            .expect("Failed to get ACPI Table from rsdp")
    }
}
