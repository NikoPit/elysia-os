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
        println!("l40");

        self.current = Some(current_process.pid);
        self.processes.insert(current_process.pid, current_process);

        println!("l45");
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
            println!("a");
            let mut current_task_id = self.current.take().unwrap();

            let mut current_task_ptr = self
                .processes
                .get_mut(&current_task_id)
                .unwrap()
                .context
                .as_ptr();
            println!("b");

            let next_task = match self.processes.get_mut(&next) {
                Some(task) => task,
                None => return,
            };
            println!("c");

            self.current = Some(next_task.pid.clone());
            println!("d");

            println!("{:?}", current_task_ptr);

            unsafe {
                Self::switch(current_task_ptr, next_task.context.as_ptr());
            }
        }
    }

    pub unsafe extern "C" fn switch(
        current: *mut multitasking::context::Context,
        next: *mut multitasking::context::Context,
    ) {
        //println!("switchz");
        arch::asm!(
            // Constructs the "Context"
            // Note: the instruction pointer have already been automatically
            // saved when we call switch();
            // because of this, we dont have to explicitly save it
            "push rbp", "push rbx", "push r12", "push r13", "push r14", "push r15"
        );
        arch::asm!(
            // Updates the context of the current process
            // to the context we've pushed to the stack
            // note to future me: the [] basically means refrence
            // &current (1st argument) = rsp (stack pointer);
            // this also saves the rsp into the rsp value of the context struct (i think lol)
            "mov [rdi], rsp",
            // rsp = &next; (rsi = second arg) (updates the stack with the context of the next process)
            "mov rsp, [rsi]"
        );
        arch::asm!(
            // pop the context of the next process
            // back into the cpu registers
            "pop r15", "pop r14", "pop r13", "pop r12", "pop rbx", "pop rbp"
        );
        println!("basf");
        // Go back to the instruction pointer that the process is at
        // (its on the stack top right now so RET will go there)
        // We dont have to explicitly push it or something
        // becuase its already been saved when we called
        // switch. so now its already sitting at the stack top
        arch::asm!("ret");
    }
}

pub extern "C" fn testz() -> ! {
    loop {
        println!("hello");
    }
}
