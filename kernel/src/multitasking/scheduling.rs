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
        let (current_ptr, current_pid) = {
            let mut curr = self.current.as_ref().unwrap().lock();
            let pid = curr.parent.lock().pid; // 锁完立刻释放
            if curr.state == State::Running {
                curr.state = State::Ready;
                self.queue.push_back(self.current.clone().unwrap());
            }
            (curr.snapshot.as_ptr(), pid)
        }; // curr 锁在这里释放

        let next_thread_arc = self.queue.pop_front().unwrap();
        let mut next_thread = next_thread_arc.lock();

        let next_pid = {
            let p = next_thread.parent.lock();
            p.pid
        };

        let next_task_snapshot_ptr = next_thread.snapshot.as_ptr();

        next_thread.state = State::Running;

        s_println!("current thread parent: {:?}", current_pid);
        s_println!("parent: {:?}", next_thread.parent.lock().pid);

        let pagetable = &mut next_thread.parent.lock().page_table;

        if current_pid != next_pid {
            // DO NOT FORGET TO SWITCH PROCESS WHEN SWITCHING THREAD U IDIOT
            pagetable.load();
            MANAGER.lock().current = Some(next_thread.parent.clone());
        }

        s_println!("The loaded pagetable: {:?}", pagetable.frame);

        self.current = Some(next_thread_arc.clone());

        unsafe {
            TSS.privilege_stack_table[0] = VirtAddr::new(next_thread.kernel_stack_top);
        }

        (current_ptr, next_task_snapshot_ptr)
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
