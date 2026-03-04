use core::ops::Deref;

use alloc::{
    boxed::Box,
    collections::{btree_map::BTreeMap, vec_deque::VecDeque},
    vec::{self, Vec},
};
use x86_64::instructions::interrupts::without_interrupts;

use crate::{
    filesystem::{path::Path, vfs::VirtualFS},
    multitasking::process::{ProcessRef, misc::ProcessID, process::Process},
    s_print, s_println,
};

#[derive(Debug, Default)]
pub struct Manager {
    pub processes: BTreeMap<ProcessID, ProcessRef>,
    pub current: Option<ProcessRef>,
    pub queue: VecDeque<ProcessRef>,
    pub zombies: Vec<ProcessRef>,
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
            self.current = Some(kernel_process.clone());
            self.processes
                .insert(kernel_process.lock().pid, kernel_process.clone());

            self.spawn(Path::new("/test.elf"));
        });
    }

    pub fn spawn(&mut self, program: Path) {
        let mut vfs = VirtualFS.lock();
        let size = vfs.file_info(program.clone()).unwrap().size;
        let layout = core::alloc::Layout::from_size_align(size as usize, 4096).unwrap();
        let ptr = unsafe { alloc::alloc::alloc(layout) };

        // 构造 Vec
        let mut buf = unsafe { alloc::vec::Vec::from_raw_parts(ptr, size as usize, size as usize) };
        s_println!("{:?}", buf.as_ptr());

        vfs.read_file(program, &mut buf).unwrap();

        for i in 1010..1050 {
            s_print!("{}\n", buf[i as usize]);
            s_print!("{}\n", ELF_HOLDER.data[i as usize]);
        }

        let process = Process::new(&buf);
        self.processes.insert(process.lock().pid, process.clone());
        self.queue.push_back(process.clone());
    }

    pub fn remove_process(&mut self, process: ProcessRef) {
        self.processes.remove(&process.lock().pid);
    }

    pub fn load_process(&mut self, process: ProcessRef) {
        let mut process_locked = process.lock();

        process_locked.page_table.load();
        self.current = Some(process.clone());
    }
}
