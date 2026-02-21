use bootloader_api::BootInfo;
use spin::Mutex;

use crate::graphics::{
    framebuffer::{FRAME_BUFFER, FrameBuffer},
    tty::TTY,
};

pub mod framebuffer;
pub mod tty;

pub fn init(boot_info: &'static mut bootloader_api::info::FrameBuffer) {
    FRAME_BUFFER.get_or_init(|| Mutex::new(FrameBuffer::new(boot_info)));

    TTY {}.draw_wallpaper();
}
