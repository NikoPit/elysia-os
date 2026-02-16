use x86_64::instructions::interrupts::without_interrupts;

use crate::multitasking::{MANAGER, context::Context, manager::Manager, process::State};

impl Manager {
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

        //next_task.page_table.load();

        return Some((current_task_ptr, next_task.context.as_ptr()));

        None
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

        next_task.page_table.load();

        return Some((next_task.context.as_ptr()));

        None
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
