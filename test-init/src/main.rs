#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    trigger_syscall();
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn trigger_syscall() {
    let syscall_number = 1; // write
    let fd = 1;
    let msg = b"GOODBYE WORLD XDDDDDD";
    let buf = msg.as_ptr();
    let count = msg.len();

    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") syscall_number,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            out("rcx") _, // 系统调用会破坏rcx和r11
            out("r11") _,
        );
    }
}
