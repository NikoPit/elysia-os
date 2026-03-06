use core::ops::Deref;

use alloc::{boxed::Box, vec};
use os_terminal::{
    Terminal,
    font::{BitmapFont, FontManager, TrueTypeFont},
};
use spin::Mutex;
use spleen_font::PSF2Font;
use vte::Parser;
use x86_64::VirtAddr;

use crate::{
    filesystem::{
        path::{Path, PathPart},
        vfs::VirtualFS,
    },
    graphics::{
        framebuffer::{Canvas, FRAME_BUFFER},
        terminal::{TERMINAL, TermRenderer},
    },
    println, s_println,
};

pub mod framebuffer;
pub mod object;
pub mod object_config;
pub mod terminal;

pub fn init(boot_info: &'static mut bootloader_api::info::FrameBuffer) {
    let canvas = FRAME_BUFFER.get_or_init(|| Mutex::new(Canvas::new(boot_info)));
    let mut terminal = TERMINAL
        .get_or_init(|| Mutex::new(Terminal::new(TermRenderer::new(canvas))))
        .lock();

    let mut vfs = VirtualFS.lock();

    let font_normal_path = Path::new("/misc/fonts/maplem~1.ttf");
    s_println!("{:?}", vfs.list_contents(Path::new("/misc/fonts")));
    let font_normal: &'static mut [u8] = Box::leak(
        Box::new(vec![
            0u8;
            vfs.file_info(font_normal_path.clone()).unwrap().size
        ])
        .into_boxed_slice(),
    );
    vfs.read_file(font_normal_path, font_normal).unwrap();

    let font_manager = TrueTypeFont::new(13.0, font_normal);

    terminal.set_font_manager(Box::new(font_manager));
    terminal.set_crnl_mapping(true);
}
