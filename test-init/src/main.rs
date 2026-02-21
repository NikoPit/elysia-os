#![no_std]
#![no_main]

use core::arch;

use elysia_os_lib::syscalls::print;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    syscall();
    syscall();
    let mut x = [0u8; 1024]; // 撑大栈空间
    core::hint::black_box(&x); // 防止被优化

    syscall();

    print("hello world from syscall wrapper!\n").unwrap();
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn syscall() {
    unsafe {
        let msg = "abc".as_bytes();
        let buf = msg.as_ptr();
        let count = msg.len();

        arch::asm!(
               "syscall",
               in("rax") 1 as isize,
               in("rdi") buf,
               in("rsi") count,
               out("rcx") _, // syscall 会覆盖 rcx
               out("r11") _, // syscall 会覆盖 r11
        )
    };
}
