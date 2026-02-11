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
        process::{Process, ProcessID},
    },
    println,
    userspace::elf_loader::Function,
};

pub struct Manager {
    pub processes: BTreeMap<ProcessID, Process>,
    pub current: Option<Process>,
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
        self.spawn(testz as Function);
    }

    // [TODO] temporary start.
    pub fn spawn(&mut self, entry_point: Function) {
        let process = Process::new((entry_point));
        self.processes.insert(process.pid, process).unwrap();
        self.queue.push(process.pid).unwrap();
    }

    fn run_next(&mut self) {
        if let Some(next) = self.queue.pop() {
            let current_task = self.current.take().unwrap();

            let next_task = match self.processes.get(&next) {
                Some(task) => task,
                None => return,
            };

            self.current = Some(*next_task);

            unsafe {
                self.switch(
                    current_task.clone().context.as_ptr(),
                    next_task.clone().context.as_ptr(),
                );
            }
        }
    }

    pub fn start(&mut self) {}

    unsafe extern "C" fn switch(
        &mut self,
        current: *mut multitasking::context::Context,
        next: *mut multitasking::context::Context,
    ) {
        arch::asm!(
            // Constructs the "Context"
            // Note: the instruction pointer have already been automatically
            // saved when we call switch();
            // because of this, we dont have to explicitly save it
            "push rbp",
            "push rbx",
            "push r12",
            "push r13",
            "push r14",
            "push r15",
            // Updates the context of the current process
            // to the context we've pushed to the stack
            // note to future me: the [] basically means refrence
            // &current (1st argument) = rsp (stack pointer);
            // this also saves the rsp into the rsp value of the context struct (i think lol)
            "mov [rdi], rsp",
            // rsp = &next; (rsi = second arg) (updates the stack with the context of the next process)
            "mov rsp, [rsi]",
            // pop the context of the next process
            // back into the cpu registers
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop rbx",
            "pop rbp",
            // Go back to the instruction pointer that the process is at
            // (its on the stack top right now so RET will go there)
            // We dont have to explicitly push it or something
            // becuase its already been saved when we called
            // switch. so now its already sitting at the stack top
            "ret"
        )
    }
}

pub extern "C" fn testz() -> ! {
    loop {
        println!("hello");
    }
}
