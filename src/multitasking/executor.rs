use core::task::{Context, Poll, Waker};

use alloc::{
    collections::{btree_map::BTreeMap, vec_deque::VecDeque},
    sync::Arc,
};
use crossbeam_queue::ArrayQueue;

use crate::multitasking::{
    task::{Task, TaskID, TaskWaker},
    waker::dummy_waker,
};

// When a task was awoken, the taskid will be pushed to the
// task queue to be executed.
pub struct Executor {
    tasks: BTreeMap<TaskID, Task>,
    task_queue: Arc<ArrayQueue<TaskID>>,
    wakers: BTreeMap<TaskID, Waker>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(128)),
            wakers: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("queue full");
    }

    pub fn run_queued_tasks(&mut self) {
        let Self {
            tasks,
            task_queue,
            wakers,
        } = self;

        while let Some(taskid) = task_queue.pop() {
            let task = match tasks.get_mut(&taskid) {
                Some(task) => task,
                None => continue,
            };
            let waker = wakers
                .entry(taskid)
                // inserts a new waker if there is no waker assigned to the task
                .or_insert_with(|| TaskWaker::new(taskid, task_queue.clone()));
            let mut context = Context::from_waker(waker);

            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    // remove the task and waker if completed
                    tasks.remove(&taskid);
                    wakers.remove(&taskid);
                }
                Poll::Pending => {}
            }
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_queued_tasks();
        }
    }
}
