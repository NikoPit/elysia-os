use acpi::{AcpiTable, AcpiTables, rsdp::Rsdp};
use alloc::sync::Arc;
use spin::Mutex;
use x86_64::structures::paging::{FrameAllocator, Mapper, Size4KiB};

use crate::{acpi::handler::ACPIHandler, println, systemcall::implementations};

pub fn init_acpi(
    mapper: Arc<Mutex<impl Mapper<Size4KiB> + 'static>>,
    frame_allocator: Arc<Mutex<impl FrameAllocator<Size4KiB> + 'static>>,
) -> AcpiTables<ACPIHandler> {
    println!("init acpi start");
    let handler = ACPIHandler::new(mapper.clone(), frame_allocator.clone());
    let rsdp =
        unsafe { Rsdp::search_for_on_bios(handler.clone()) }.expect("Failed searching for Rsdp");
    unsafe {
        AcpiTables::from_rsdp(handler, rsdp.physical_start)
            .expect("Failed to get ACPI Table from rsdp")
    }
}
