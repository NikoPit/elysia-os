use core::panic;

use acpi::{AcpiError, AcpiTable, AcpiTables, rsdp::Rsdp};
use alloc::sync::Arc;
use spin::Mutex;
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Size4KiB};

use crate::{
    acpi::handler::ACPIHandler,
    memory::paging::BootinfoFrameAllocator,
    println,
    systemcall::{error, implementations},
};

pub fn init_acpi(
    mapper: Arc<Mutex<OffsetPageTable<'static>>>,
    frame_allocator: Arc<Mutex<BootinfoFrameAllocator>>,
) -> AcpiTables<ACPIHandler> {
    let handler = ACPIHandler::new(mapper.clone(), frame_allocator.clone());
    let rsdp = match unsafe { Rsdp::search_for_on_bios(handler) } {
        Ok(value) => value,
        Err(err) => {
            panic!("{:?}", err);
        }
    };
    unsafe {
        match AcpiTables::from_rsdt(handler, 1, rsdp.get().rsdt_address() as usize) {
            Ok(value) => value,
            Err(error) => {
                panic!("{:?}", error)
            }
        }
    }
}
