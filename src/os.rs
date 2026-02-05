use crate::{
    acpi::init::init_acpi,
    gdt::init_gdt,
    hardware_interrupt::{PIC_1_OFFSET, PIC_2_OFFSET},
    heap::init_heap,
    interrupts::init_idt,
    paging::{BootinfoFrameAllocator, init_mapper},
    systemcall::entry::init_syscall,
    vga_print::Printer,
};
use bootloader::BootInfo;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::*;
use uart_16550::SerialPort;
use x86_64::{
    VirtAddr,
    instructions::interrupts,
    structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Size4KiB, mapper},
};

lazy_static! {
    pub static ref ELYSIA_OS: Mutex<OS> = Mutex::new(OS::new());
}

pub struct OS {
    pub printer: Printer,
    pub serial_port: SerialPort,
    pub pics: ChainedPics,
    pub phys_mem_offset: Option<VirtAddr>,
}

impl OS {
    pub fn new() -> Self {
        Self {
            printer: Printer::new(),
            serial_port: {
                let mut serial_port = unsafe { SerialPort::new(0x3F8) };
                serial_port.init();
                serial_port
            },
            pics: unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) },
            phys_mem_offset: None,
        }
    }

    pub fn init(
        &mut self,
        bootinfo: &'static BootInfo,
        mapper: &'static mut impl Mapper<Size4KiB>,
        frame_allocator: &'static mut impl FrameAllocator<Size4KiB>,
    ) {
        init_gdt();
        init_idt();
        init_heap(mapper, frame_allocator).expect("Heap init failed.");
        init_acpi(mapper, frame_allocator);

        self.phys_mem_offset = Some(VirtAddr::new(bootinfo.physical_memory_offset));

        unsafe { self.pics.initialize() };
        interrupts::enable();

        init_syscall();
    }
}

pub fn get_os_no_interrupt<F>(func: F)
where
    F: FnOnce(MutexGuard<'static, OS>),
{
    interrupts::without_interrupts(|| func(get_os()));
}

pub fn get_os() -> MutexGuard<'static, OS> {
    ELYSIA_OS.lock()
}
