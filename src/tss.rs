use lazy_static::lazy_static;
use x86_64::{VirtAddr, structures::tss::TaskStateSegment};

pub const DOUBLE_FAULT_IST_LOCATION: u16 = 0;

lazy_static! {
    // a TSS is used to store the interrupt_stack_table (IST) and other stuff
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[DOUBLE_FAULT_IST_LOCATION as usize] = {
            const STACK_SIZE: usize = 4096 * 5;

            // doing some dark magic wizardy to create a stack by declaring a
            // static mut array
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            // load the stack created with the dark magic wizardy above with a refrence to the
            // stack or something idk this shit is too dark magic to me
            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE as u64;
            stack_end
        };

        tss
    };
}
