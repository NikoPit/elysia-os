use bootloader_api::BootInfo;
use spin::Mutex;

use crate::graphics::framebuffer::{FRAME_BUFFER, FrameBuffer};

pub mod framebuffer;
pub mod tty;

pub fn init(boot_info: &'static mut bootloader_api::info::FrameBuffer) {
    let mut fb = FRAME_BUFFER
        .get_or_init(|| Mutex::new(FrameBuffer::new(boot_info)))
        .lock();

    for x in 0..100 {
        for y in 0..100 {
            fb.write_pixel(x, y, 255, 0, 0);
        }
    }
}
