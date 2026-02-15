use x86_64::{
    instructions::interrupts::without_interrupts,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{
    debug_exit::debug_exit,
    multitasking::{MANAGER, manager::Manager, scheduling::run_next},
    os::get_os,
    print, println, s_print,
};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum HardwareInterrupt {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl HardwareInterrupt {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub trait HardwareInterruptHandler {
    const HARDWARE_INTERRUPT: HardwareInterrupt;

    fn handle_hardware_interrupt_unwrapped(_stack_frame: InterruptStackFrame);

    extern "x86-interrupt" fn handle_hardware_interrupt(_stack_frame: InterruptStackFrame) {
        Self::handle_hardware_interrupt_unwrapped(_stack_frame);
        notify_end_of_interrupt(Self::HARDWARE_INTERRUPT);
    }
}

pub fn notify_end_of_interrupt(interrupt: HardwareInterrupt) {
    unsafe {
        get_os().pics.notify_end_of_interrupt(interrupt.as_u8());
    }
}

#[macro_export]
macro_rules! register_hardware_interrupt {
    ($idt:expr, $interrupt:expr, $handler:ty) => {
        $idt[$interrupt.as_u8()].set_handler_fn(<$handler>::handle_hardware_interrupt);
    };
}

pub fn init_hardware_interrupts(idt: &mut InterruptDescriptorTable) {
    register_hardware_interrupt!(idt, HardwareInterrupt::Timer, TimerHandler);
}

struct TimerHandler;

impl HardwareInterruptHandler for TimerHandler {
    const HARDWARE_INTERRUPT: HardwareInterrupt = HardwareInterrupt::Timer;

    fn handle_hardware_interrupt_unwrapped(_stack_frame: InterruptStackFrame) {
        notify_end_of_interrupt(Self::HARDWARE_INTERRUPT);
        s_print!(".");
        // NOTE: DO NOT call context_switch deep within a call stack
        // because it will messup the stack
        let targets = {
            without_interrupts(|| {
                let mut manager = MANAGER.lock();
                manager.next()
            })
        }
        .unwrap();

        unsafe {
            Manager::context_switch(targets.0, targets.1);
        }
    }
}
