use core::{arch, ops::Deref};

use alloc::{
    collections::{btree_map::BTreeMap, vec_deque::VecDeque},
    sync::Arc,
    vec::Vec,
};
use crossbeam_queue::ArrayQueue;
use x86_64::{
    instructions::interrupts::{self, without_interrupts},
    registers::{
        control,
        rflags::{self, RFlags},
    },
    structures::paging::{OffsetPageTable, PageTable, Size4KiB},
};

use crate::{
    hardware_interrupt::notify_end_of_interrupt,
    misc::hlt_loop,
    multitasking::{
        self, MANAGER,
        context::Context,
        process::{self, Process, ProcessID, State},
        scheduling::run_next,
        yielding::{BlockType, BlockedQueues, WakeType},
    },
    print, println, s_print, s_println,
    userspace::elf_loader::Function,
};

pub struct Manager {
    pub processes: BTreeMap<ProcessID, Process>,
    pub current: Option<ProcessID>,
    pub queue: VecDeque<ProcessID>,
    pub zombies: Vec<ProcessID>,
    pub blocked_queues: BlockedQueues,

    pub idle_process: Option<ProcessID>,
}

impl Default for Manager {
    fn default() -> Self {
        Self {
            processes: BTreeMap::new(),
            idle_process: None,
            zombies: Vec::new(),
            current: None,
            blocked_queues: BlockedQueues::new(),
            queue: (VecDeque::new()),
        }
    }
}

impl Manager {
    pub fn init(&mut self) {
        without_interrupts(|| {
            let kernel_process = Process::default();
            let pid = kernel_process.pid;

            let idle_process = Process::new(idle as Function);

            self.current = Some(pid);
            self.processes.insert(pid, kernel_process);

            self.idle_process = Some(idle_process.pid);
            self.processes
                .insert(idle_process.pid.clone(), idle_process);

            self.spawn(test3 as Function);
            self.spawn(testz as Function);
            self.spawn(test2 as Function);
        });
    }

    // [TODO] temporary start.
    pub fn spawn(&mut self, entry_point: Function) {
        let process = Process::new(entry_point);
        let pid = process.pid.clone();
        self.processes.insert(process.pid, process);
        self.queue.push_back(pid);
    }

    pub fn clean_zombies(&mut self) {
        for (ele) in self.zombies.drain(..) {
            self.processes.remove(&ele);
            self.current.take_if(|p| *p == ele);
        }
    }

    pub fn block_current(&mut self, block_type: BlockType) {
        let mut current = self.processes.get_mut(&self.current.unwrap()).unwrap();

        current.state = process::State::Blocked(block_type);
        // TODO make this work. to future me: uncomment it and u will see why
        //self.queue.into_iter().filter(|p| *p != current.pid.clone());

        match block_type {
            BlockType::WakeRequired(wake_type) => match wake_type {
                WakeType::Keyboard => self.blocked_queues.keyboard.push_back(current.pid.clone()),
                WakeType::IO => self.blocked_queues.io.push_back(current.pid.clone()),
            },
            _ => {}
        }

        run_next();
    }
}

pub fn schedule() {
    let targets = {
        without_interrupts(|| {
            let mut manager = MANAGER.lock();
            manager.next()
        })
    }
    .unwrap();

    notify_end_of_interrupt(crate::hardware_interrupt::HardwareInterrupt::Timer);

    unsafe {
        Manager::context_switch(targets.0, targets.1);
    }
}

pub extern "C" fn test3() -> ! {
    loop {
        print!("1");
    }
}

pub extern "C" fn test2() -> ! {
    loop {
        print!("2");
    }
}

pub extern "C" fn testz() -> ! {
    loop {
        print!("3");
    }
}

pub extern "C" fn idle() -> ! {
    println!("idle");
    hlt_loop()
}
