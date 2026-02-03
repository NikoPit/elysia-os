use core::task::{Context, Poll};

use alloc::collections::vec_deque::VecDeque;

use crate::multitasking::{task::Task, waker::dummy_waker};

pub struct Executor {
    tasks: VecDeque<Task>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);

            match task.poll(&mut context) {
                // Task completed. dont do anything
                Poll::Ready(()) => {}

                // Move the task back
                Poll::Pending => self.tasks.push_back(task),
            }
        }
    }
}
