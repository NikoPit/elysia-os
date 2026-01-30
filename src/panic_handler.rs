// Implement panic handler beacuse the original implementaion is from the std lib, which doesnt
// exist anymore.

use core::panic::PanicInfo;

use crate::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use crate::{
        debug_exit::{QemuExitCode, debug_exit},
        s_print, s_println,
    };

    s_println!("[Failed]\n");
    s_println!("Error:\n{}\n", _info);

    debug_exit(QemuExitCode::Failed);

    loop {}
}
