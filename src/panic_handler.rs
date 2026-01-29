// Implement panic handler beacuse the original implementaion is from the std lib, which doesnt
// exist anymore.

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
