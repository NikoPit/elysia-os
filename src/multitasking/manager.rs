use core::arch;

use alloc::{collections::btree_map::BTreeMap, sync::Arc};
use crossbeam_queue::ArrayQueue;
use x86_64::{
    registers::control,
    structures::paging::{OffsetPageTable, PageTable, Size4KiB},
};

use crate::{
    multitasking::{
        self,
        context::Context,
        process::{self, Process, ProcessID},
    },
    println,
    userspace::elf_loader::Function,
};

pub struct Manager {
    pub processes: BTreeMap<ProcessID, Process>,
    pub current: Option<ProcessID>,
    pub queue: Arc<ArrayQueue<ProcessID>>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            current: None,
            queue: Arc::new(ArrayQueue::new(128)),
        }
    }

    pub fn init(&mut self) {
        let current_process = Process {
            pid: ProcessID::new(),
            context: Context::empty(),
        };

        self.current = Some(current_process.pid);
        self.processes.insert(current_process.pid, current_process);

        self.spawn(testz as Function);
        self.run_next();
    }

    // [TODO] temporary start.
    pub fn spawn(&mut self, entry_point: Function) {
        let process = Process::new((entry_point));
        let pid = process.pid.clone();
        self.processes.insert(process.pid, process);
        self.queue.push(pid);
    }

    fn run_next(&mut self) {
        if let Some(next) = self.queue.pop() {
            let mut current_task_id = self.current.take().unwrap();

            let mut current_task_ptr = self
                .processes
                .get_mut(&current_task_id)
                .unwrap()
                .context
                .as_ptr();

            let next_task = match self.processes.get_mut(&next) {
                Some(task) => task,
                None => return,
            };

            self.current = Some(next_task.pid.clone());

            unsafe {
                Self::context_switch(current_task_ptr, next_task.context.as_ptr());
            }
        }
    }
}

pub extern "C" fn testz() {
    println!("hello from process 1!!!");
    println!("YOOO");
}
