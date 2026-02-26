use alloc::{
    collections::{btree_map::BTreeMap, vec_deque::VecDeque},
    vec::Vec,
};
use x86_64::instructions::interrupts::without_interrupts;

use crate::{
    misc::{self, hlt_loop},
    multitasking::{
        MANAGER,
        process::{
            ProcessRef,
            process::{self, Process, ProcessID},
        },
        yielding::{BlockType, BlockedQueues, WakeType},
    },
    print, println, s_println,
};

#[derive(Debug, Default)]
pub struct Manager {
    pub processes: BTreeMap<ProcessID, ProcessRef>,
    pub current: Option<ProcessRef>,
    pub queue: VecDeque<ProcessRef>,
    pub zombies: Vec<ProcessRef>,
    pub blocked_queues: BlockedQueues,

    pub idle_process: Option<ProcessRef>,
}

#[repr(align(8))]
struct AlignedElf {
    data: [u8; include_bytes!("../../../../libc-test/test.elf").len()],
}

static ELF_HOLDER: AlignedElf = AlignedElf {
    data: *include_bytes!("../../../../libc-test/test.elf"),
};

impl Manager {
    pub fn init(&mut self) {
        without_interrupts(|| {
            let kernel_process = Process::empty();
            // TODO: delete the idle proecss or let it fucking work with all that shit
            let idle_process = Process::empty();

            self.current = Some(kernel_process.clone());
            self.processes
                .insert(kernel_process.lock().pid, kernel_process.clone());

            self.idle_process = Some(idle_process.clone());
            self.processes
                .insert(idle_process.lock().pid, idle_process.clone());

            // TODO: remove these test processes
            self.spawn(&ELF_HOLDER.data);
        });
    }

    pub fn spawn(&mut self, program: &[u8]) {
        let process = Process::new(program);
        let pid = process.lock().pid;
        s_println!(
            "process pagetable frame at spawn() {:?}",
            process.lock().page_table.frame
        );
        self.processes.insert(process.lock().pid, process.clone());
        self.queue.push_back(process.clone());
        s_println!(
            "process pagetable got from self.processes {:?}",
            self.processes.get(&pid).unwrap().lock().page_table.frame
        );
        s_println!("queue: {:?}", self.queue);
    }

    pub fn clean_zombies(&mut self) {
        for zombie in self.zombies.drain(..) {
            self.processes.remove(&zombie.lock().pid);
            self.current.take_if(|p| p.lock().pid == zombie.lock().pid);
        }
    }

    pub fn block_current_unwrappped(&mut self, block_type: BlockType) {
        let current = self.current.clone().unwrap();

        current.lock().state = process::State::Blocked(block_type);
        //self.queue.into_iter().filter(|p| *p != current.pid.clone());

        match block_type {
            BlockType::WakeRequired(wake_type) => match wake_type {
                WakeType::Keyboard => self.blocked_queues.keyboard.push_back(current),
                WakeType::IO => self.blocked_queues.io.push_back(current),
            },
            _ => {}
        }

        //run_next();
    }
}

pub fn block_current(block_type: BlockType) {
    MANAGER.lock().block_current_unwrappped(block_type);
    // TODO
    //run_next(InterruptStackFrame::new(fwefwefas, code_segment, cpu_flags, stack_pointer, stack_segment));
}
