use core::arch::naked_asm;

use alloc::{
    collections::{btree_map::BTreeMap, vec_deque::VecDeque},
    sync::Arc,
};
use spin::Mutex;

use crate::{
    multitasking::thread::{self, ThreadRef, misc::ThreadID, thread::Thread},
    s_println,
};

#[derive(Default, Debug)]
pub struct ThreadManager {
    pub threads: BTreeMap<ThreadID, ThreadRef>,
    pub current: Option<ThreadRef>,
    pub queue: VecDeque<ThreadRef>,
    pub idle_thread: Option<ThreadRef>,
}

impl ThreadManager {
    pub fn init(&mut self) {
        self.current = Some(Thread::empty());
    }

    pub fn spawn(&mut self, thread: Thread) -> ThreadRef {
        s_println!("someone called spawn thread or smth");
        let id = thread.id;
        let thread = Arc::new(Mutex::new(thread));

        self.threads.insert(id, thread);

        let thread = self.threads.get_mut(&id).unwrap();

        self.queue.push_back(thread.clone());
        thread.clone()
    }
}
