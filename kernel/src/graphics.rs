use bootloader_api::BootInfo;
use spin::Mutex;

use crate::graphics::{
    framebuffer::{Canvas, FRAME_BUFFER},
    tty::TTY,
};

pub mod framebuffer;
pub mod tty;

pub fn init(boot_info: &'static mut bootloader_api::info::FrameBuffer) {
    FRAME_BUFFER.get_or_init(|| Mutex::new(Canvas::new(boot_info)));

    TTY {}.draw_wallpaper();
}
