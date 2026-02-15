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
        blocked::{BlockType, BlockedQueues, WakeType},
        context::Context,
        process::{self, Process, ProcessID, State},
    },
    println, s_print, s_println,
    userspace::elf_loader::Function,
};

#[derive(Debug)]
pub struct Manager {
    pub processes: BTreeMap<ProcessID, Process>,
    pub current: Option<ProcessID>,
    pub queue: VecDeque<ProcessID>,
    pub zombies: Vec<ProcessID>,
    pub blocked_queues: BlockedQueues,

    pub idle_process: Option<ProcessID>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            idle_process: None,
            zombies: Vec::new(),
            current: None,
            blocked_queues: BlockedQueues::new(),
            queue: (VecDeque::new()),
        }
    }

    pub fn init(&mut self) {
        without_interrupts(|| {
            let kernel_process = Process::default();
            let pid = kernel_process.pid.clone();

            let idle_process = Process::new(idle as Function);

            self.current = Some(pid);
            self.processes.insert(pid, kernel_process);

            self.idle_process = Some(idle_process.pid.clone());
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

    fn clean_zombies(&mut self) {
        for (ele) in self.zombies.drain(..) {
            self.processes.remove(&ele);
            self.current.take_if(|p| *p == ele);
        }
    }

    /// runs the next process. called from a zombie process
    pub fn next_zombie(&mut self) -> Option<(*mut Context)> {
        self.clean_zombies();

        let mut next_task = if let Some(next) = self.queue.pop_front() {
            let next_task = match self.processes.get_mut(&next) {
                Some(task) => task,
                // Possibly zombie task
                None => return None,
            };

            next_task
        } else {
            // call the idle process if there is nothing to do
            match self.processes.get_mut(&self.idle_process.unwrap()) {
                Some(task) => task,
                None => panic!("This isnt supposed to happen"),
            }
        };

        self.current = Some(next_task.pid.clone());

        return Some((next_task.context.as_ptr()));

        None
    }

    pub fn next(&mut self) -> Option<(*mut Context, *mut Context)> {
        let mut current_task_id = self.current.take().unwrap();

        let mut current_task_ptr = {
            let current_task = self.processes.get_mut(&current_task_id).unwrap();

            if current_task.state == State::Running {
                current_task.state = State::Ready;
                self.queue.push_back(current_task_id);
            }

            self.processes
                .get_mut(&current_task_id)
                .unwrap()
                .context
                .as_ptr()
        };

        let next_task = if let Some(next) = self.queue.pop_front() {
            let next_task = match self.processes.get_mut(&next) {
                Some(task) => task,
                // Possibly zombie task
                None => return None,
            };

            next_task
        } else {
            match self.processes.get_mut(&self.idle_process.unwrap()) {
                Some(task) => task,
                None => panic!("This isnt supposed to happen"),
            }
        };

        next_task.state = State::Running;

        self.current = Some(next_task.pid.clone());

        return Some((current_task_ptr, next_task.context.as_ptr()));

        None
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

pub fn run_next() {
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

/// runs the next process. called from a zombie process
pub fn run_next_zombie() {
    let target = match {
        without_interrupts(|| {
            let mut manager = MANAGER.lock();
            manager
        })
    }
    .next_zombie()
    {
        Some(val) => val,
        None => {
            return;
        }
    };
    unsafe {
        Manager::context_switch_zombie(target);
    }
}

pub extern "C" fn test3() -> ! {
    loop {}
}

pub extern "C" fn test2() -> ! {
    loop {}
}

pub extern "C" fn testz() -> ! {
    loop {}
}

pub extern "C" fn idle() -> ! {
    println!("idle");
    hlt_loop()
}
