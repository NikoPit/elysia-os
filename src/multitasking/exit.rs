use crate::{misc::hlt_loop, multitasking::MANAGER, println};

// it should never return because its placed on the stack bottom
pub extern "C" fn exit_handler() -> ! {
    println!("todo, task exited");

    hlt_loop()
}
