use x86_64::instructions::interrupts::without_interrupts;

use crate::{
    multitasking::{
        MANAGER,
        context::Context,
        manager::Manager,
        process::State,
        switch::{context_switch, context_switch_zombie},
    },
    s_print,
};

impl Manager {
    fn run_next_unwrapped(&mut self) -> (*mut Context, *mut Context) {
        let current_task_id = self.current.take().unwrap();

        let current_task_ptr = {
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
            self.processes.get_mut(&next).unwrap()
        } else {
            self.processes.get_mut(&self.idle_process.unwrap()).unwrap()
        };

        next_task.state = State::Running;

        self.current = Some(next_task.pid);

        (current_task_ptr, next_task.context.as_ptr())
    }

    /// picks the next process. called from a zombie process
    fn run_next_zombie_unwrapped(&mut self) -> *mut Context {
        self.clean_zombies();

        let next_task = if let Some(next) = self.queue.pop_front() {
            self.processes.get_mut(&next).unwrap()
        } else {
            // call the idle process if there is nothing to do
            match self.processes.get_mut(&self.idle_process.unwrap()) {
                Some(task) => task,
                None => panic!("This isnt supposed to happen"),
            }
        };

        next_task.state = State::Running;

        self.current = Some(next_task.pid);

        next_task.context.as_ptr()
    }
}

pub fn run_next() {
    let targets = {
        without_interrupts(|| {
            let mut manager = MANAGER.lock();
            manager.run_next_unwrapped()
        })
    };

    unsafe {
        context_switch(targets.0, targets.1);
    }
}

/// runs the next process. called from a zombie process
pub fn run_next_zombie() {
    let target = without_interrupts(|| {
        let mut manager = MANAGER.lock();
        manager.run_next_zombie_unwrapped()
    });

    unsafe {
        context_switch_zombie(target);
    }
}
