use x86_64::instructions::hlt;

pub fn hlt_loop() -> ! {
    loop {
        hlt();
    }
}
