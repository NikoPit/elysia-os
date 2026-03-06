use core::ops::Deref;

use alloc::boxed::Box;
use os_terminal::{
    Terminal,
    font::{BitmapFont, FontManager},
};
use spin::Mutex;
use spleen_font::PSF2Font;

use crate::graphics::{
    framebuffer::{Canvas, FRAME_BUFFER},
    terminal::{TERMINAL, Tty},
};

pub mod framebuffer;
pub mod object;
pub mod object_config;
pub mod terminal;

pub static FONT: &[u8] = include_bytes!("../../../maplemono.psf");

pub fn init(boot_info: &'static mut bootloader_api::info::FrameBuffer) {
    FRAME_BUFFER.get_or_init(|| Mutex::new(Canvas::new(boot_info)));
    let terminal = TERMINAL.get_or_init(|| Mutex::new(Terminal::new(Tty::new())));

    for i in 0..100 {
        FRAME_BUFFER
            .get()
            .unwrap()
            .lock()
            .write_pixel(i, i, (0, 0, 0));
    }
    FRAME_BUFFER.get().unwrap().lock().flush();

    terminal.lock().set_font_manager(Box::new(BitmapFont));
    terminal.lock().process(b"asdadas");
}
