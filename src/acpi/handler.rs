use core::{iter::Map, ptr::NonNull};

use acpi::{Handler, PhysicalMapping, address};
use alloc::sync::Arc;
use spin::Mutex;
use x86_64::{
    PhysAddr, VirtAddr,
    instructions::port::Port,
    structures::paging::{self, FrameAllocator, Mapper, Page, PageTableFlags, PhysFrame, Size4KiB},
};

use crate::{
    os::get_os, read_addr, read_port, systemcall::implementations::utils::SystemCallImpl,
    write_addr, write_port,
};

#[derive(Clone)]
pub struct ACPIHandler {
    mapper: Arc<Mutex<dyn Mapper<Size4KiB>>>,
    frame_allocator: Arc<Mutex<dyn FrameAllocator<Size4KiB>>>,
}

impl ACPIHandler {
    pub fn new(
        mapper: Arc<Mutex<impl Mapper<Size4KiB> + 'static>>,
        frame_allocator: Arc<Mutex<impl FrameAllocator<Size4KiB> + 'static>>,
    ) -> Self {
        Self {
            mapper: mapper.clone(),
            frame_allocator: frame_allocator.clone(),
        }
    }
}

impl Handler for ACPIHandler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> acpi::PhysicalMapping<Self, T> {
        let virt_addr_number = 0x6235461612;
        let virt_addr = VirtAddr::new(virt_addr_number);

        let frame: PhysFrame<Size4KiB> =
            PhysFrame::containing_address(PhysAddr::new(physical_address as u64));
        let page: Page = Page::containing_address(virt_addr);
        let flags = PageTableFlags::WRITABLE | PageTableFlags::PRESENT;

        let virt_addr_nonnull = NonNull::new(page.start_address().as_u64() as *mut T);

        unsafe {
            // self.mapper
            //   .map_to(page, frame, flags, &mut self.frame_allocator)
            // .expect("Failed mapping on apic Handler")
            //.flush();
        }

        PhysicalMapping {
            physical_start: frame.start_address().as_u64() as usize,
            mapped_length: size,
            handler: self.clone(),
            region_length: size,
            virtual_start: virt_addr_nonnull.unwrap(),
        }
    }

    fn unmap_physical_region<T>(region: &PhysicalMapping<Self, T>) {}

    fn read_u8(&self, address: usize) -> u8 {
        unsafe { read_addr!(address, u8) }
    }

    fn read_u16(&self, address: usize) -> u16 {
        unsafe { read_addr!(address, u16) }
    }

    fn read_u32(&self, address: usize) -> u32 {
        unsafe { read_addr!(address, u32) }
    }

    fn read_u64(&self, address: usize) -> u64 {
        unsafe { read_addr!(address, u64) }
    }

    fn read_io_u8(&self, port: u16) -> u8 {
        unsafe { read_port!(port) }
    }

    fn read_io_u16(&self, port: u16) -> u16 {
        unsafe { read_port!(port) }
    }

    fn read_io_u32(&self, port: u16) -> u32 {
        unsafe { read_port!(port) }
    }

    fn write_u8(&self, address: usize, value: u8) {
        unsafe { write_addr!(address, u8, value) }
    }

    fn write_u16(&self, address: usize, value: u16) {
        unsafe { write_addr!(address, u16, value) }
    }

    fn write_u32(&self, address: usize, value: u32) {
        unsafe { write_addr!(address, u32, value) }
    }

    fn write_u64(&self, address: usize, value: u64) {
        unsafe { write_addr!(address, u64, value) }
    }

    fn write_io_u8(&self, port: u16, value: u8) {
        unsafe { write_port!(port, value) }
    }

    fn write_io_u16(&self, port: u16, value: u16) {
        unsafe { write_port!(port, value) }
    }

    fn write_io_u32(&self, port: u16, value: u32) {
        unsafe { write_port!(port, value) }
    }

    fn write_pci_u8(&self, address: acpi::PciAddress, offset: u16, value: u8) {
        unimplemented!()
    }

    fn write_pci_u16(&self, address: acpi::PciAddress, offset: u16, value: u16) {
        unimplemented!()
    }

    fn write_pci_u32(&self, address: acpi::PciAddress, offset: u16, value: u32) {
        unimplemented!()
    }

    fn read_pci_u8(&self, address: acpi::PciAddress, offset: u16) -> u8 {
        unimplemented!()
    }

    fn read_pci_u16(&self, address: acpi::PciAddress, offset: u16) -> u16 {
        unimplemented!()
    }

    fn read_pci_u32(&self, address: acpi::PciAddress, offset: u16) -> u32 {
        unimplemented!()
    }

    fn nanos_since_boot(&self) -> u64 {
        0
    }

    fn stall(&self, microseconds: u64) {
        unimplemented!()
    }

    fn sleep(&self, milliseconds: u64) {
        unimplemented!()
    }

    fn create_mutex(&self) -> acpi::Handle {
        unimplemented!()
    }

    fn release(&self, mutex: acpi::Handle) {
        unimplemented!()
    }

    fn acquire(&self, mutex: acpi::Handle, timeout: u16) -> Result<(), acpi::aml::AmlError> {
        unimplemented!()
    }
}
