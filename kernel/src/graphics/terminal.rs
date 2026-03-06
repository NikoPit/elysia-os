use core::fmt::{Arguments, Write};

use conquer_once::spin::OnceCell;
use os_terminal::Terminal;
use spin::Mutex;

use crate::graphics::framebuffer::{Canvas, FRAME_BUFFER};

pub type Color = (u8, u8, u8);

pub static TERMINAL: OnceCell<Mutex<Terminal<Tty>>> = OnceCell::uninit();

pub struct Tty<'a> {
    canvas: &'a Mutex<Canvas>,
    pub width: u32,
    pub height: u32,
}

impl<'a> Tty<'a> {
    pub fn new() -> Self {
        let width = FRAME_BUFFER.get().unwrap().lock().width;
        let height = FRAME_BUFFER.get().unwrap().lock().height;
        Self {
            canvas: FRAME_BUFFER.get().unwrap(),
            width,
            height,
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::graphics::terminal::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    TERMINAL.get().unwrap().lock().write_fmt(args).unwrap();
}

use os_terminal::DrawTarget;

impl<'a> DrawTarget for Tty<'a> {
    fn size(&self) -> (usize, usize) {
        (self.width as usize, self.height as usize)
    }

    #[inline(always)]
    fn draw_pixel(&mut self, x: usize, y: usize, rgb: os_terminal::Rgb) {
        self.canvas.lock().write_pixel(x, y, (rgb.0, rgb.1, rgb.2));
    }
}
