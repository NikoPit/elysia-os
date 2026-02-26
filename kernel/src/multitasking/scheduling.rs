use core::panic;

use x86_64::{VirtAddr, instructions::interrupts::without_interrupts};

use crate::{
    misc::snapshot::Snapshot,
    multitasking::{
        MANAGER,
        process::{manager::Manager, process::State},
        thread::{THREAD_MANAGER, manager::ThreadManager, snapshot::ThreadSnapshot},
    },
    s_println,
    tss::TSS,
};

impl ThreadManager {
    fn run_next_unwrapped(&mut self) -> (*mut ThreadSnapshot, *mut ThreadSnapshot) {
        let current_task_arc = self.current.as_mut().unwrap().clone();
        let mut current_task = current_task_arc.lock();

        if current_task.state == State::Running {
            current_task.state = State::Ready;
            self.queue.push_back(current_task_arc.clone());
        }

        let next_task = if let Some(next) = self.queue.pop_front() {
            next.clone()
        } else {
            unimplemented!();
        };

        let mut next_task_mutex = next_task.lock();
        let binding = &mut next_task_mutex.parent.clone();
        let mut locked_binding_pid_shit = {
            let locked = binding.lock();
            locked.pid.clone()
        };

        s_println!("parent: {:?}", binding.lock().pid);

        let pagetable = &mut binding.lock().page_table;

        if current_task.parent.lock().pid != locked_binding_pid_shit {
            // DO NOT FORGET TO SWITCH PROCESS WHEN SWITCHING THREAD U IDIOT
            pagetable.load();
            MANAGER.lock().current = Some(next_task_mutex.parent.clone());
        }
        pagetable.load();
        next_task_mutex.state = State::Running;

        s_println!("process is {:?}", next_task_mutex.id.0);
        s_println!("The loaded pagetable: {:?}", pagetable.frame);

        self.current = Some(next_task.clone());

        unsafe {
            TSS.privilege_stack_table[0] = VirtAddr::new(next_task_mutex.kernel_stack_top);
        }

        (
            current_task.snapshot.as_ptr(),
            next_task_mutex.snapshot.as_ptr(),
        )
    }

    /// picks the next process. called from a zombie process
    fn run_next_zombie_unwrapped(&mut self) -> *mut ThreadSnapshot {
        unimplemented!();
        //self.clean_zombies();

        //let next_task = if let Some(next) = self.queue.pop_front() {
        //  self.processes.get_mut(&next).unwrap()
        //} else {
        //  // call the idle process if there is nothing to do
        //match self.processes.get_mut(&self.idle_process.unwrap()) {
        //  Some(task) => task,
        //None => panic!("This isnt supposed to happen"),
        //}
        //};

        //     next_task.state = State::Running;

        //   self.current = Some(next_task.pid);

        // unsafe {
        //   TSS.privilege_stack_table[0] = next_task.kernel_stack_top;
        //}

        //next_task.context.as_ptr()
    }
}

pub fn run_next(snapshot: &mut Snapshot) {
    let (current, next) = {
        without_interrupts(|| {
            let mut manager = THREAD_MANAGER.get().unwrap().lock();
            manager.run_next_unwrapped()
        })
    };

    unsafe { (*next).switch_from(Some(current.as_mut().unwrap()), Some(snapshot)) };
}

/// runs the next process. called from a zombie process
pub fn run_next_zombie() {
    let next = without_interrupts(|| {
        let mut manager = THREAD_MANAGER.get().unwrap().lock();
        manager.run_next_zombie_unwrapped()
    });

    s_println!("next task: {:?}", next);

    unsafe {
        (*next).switch_from(None, None);
    }
}
