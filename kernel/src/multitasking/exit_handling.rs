use crate::{
    misc::hlt_loop,
    multitasking::{MANAGER, scheduling::run_next_zombie},
    println,
};

// it should never return because its placed on the stack bottom
pub extern "C" fn exit_handler() -> ! {
    // push the current process to zombies list
    if let Some(mut manager) = MANAGER.try_lock() {
        if let Some(pid) = manager.current {
            manager.zombies.push(pid);
        }
    } else {
        println!("manager locked.")
    }

    run_next_zombie();
    hlt_loop()
}
