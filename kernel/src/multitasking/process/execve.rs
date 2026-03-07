use alloc::{str::pattern::StrSearcher, string::String, vec::Vec};

use crate::{
    filesystem::{path::Path, vfs::VirtualFS},
    multitasking::{
        process::{
            Process,
            misc::{init_objects, init_stack_layout},
        },
        thread::{
            THREAD_MANAGER,
            snapshot::{ThreadSnapshot, ThreadSnapshotType},
            thread::Thread,
        },
    },
    userspace::elf_loader::load_elf,
};

impl Process {
    pub fn execve(&mut self, path: Path, args: Vec<String>) {
        self.addrspace.clean();

        let thread = THREAD_MANAGER
            .get()
            .unwrap()
            .lock()
            .current
            .clone()
            .unwrap();

        let mut program =
            alloc::vec![0u8; VirtualFS.lock().file_info(path.clone()).unwrap().size as usize];
        VirtualFS.lock().read_file(path.clone(), &mut program);

        let mut stack_builder = self.addrspace.allocate_user(16).1;
        let program = load_elf(&mut self.addrspace, &mut program);

        // Reallocates the kernel stack top (just in case)
        self.kernel_stack_top = self.addrspace.allocate_kernel(16).1.finish();

        assert!(!program.is_pie(), "Pie program is not supported for now");

        init_stack_layout(&mut stack_builder, &program);

        thread.lock().snapshot = ThreadSnapshot::new(
            program.entry_point() as u64,
            &mut self.addrspace,
            stack_builder.finish().as_u64(),
            ThreadSnapshotType::Thread,
        );

        init_objects(&mut self.objects);
    }
}
