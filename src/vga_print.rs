use core::cell;
use core::fmt::{self, Write};

use crate::os::{get_os, get_os_no_interrupt};
use crate::test;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
// TODO: make it volatile
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellColor(u8);

impl CellColor {
    // How VGA Buffer take a cell color:
    // A CellColor is a byte (u8)
    // And the first 4 bits are used to store its foreground
    // the last 4 bits are used to store its background
    // for example, the cell color 0x4f
    // "4" is the foreground color
    // "f" is the background color
    // (hopefully, im not so sure and it might be incorrect)
    pub fn new(foreground: VgaColor, background: VgaColor) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VgaCell {
    character: u8,
    color: CellColor,
}

impl VgaCell {
    pub fn new(character: u8, color: CellColor) -> Self {
        Self { character, color }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Printer {
    row: usize,
    column: usize,

    buffer_tracker: [[u8; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Printer {
    pub fn new() -> Self {
        Self {
            row: 0,
            column: 0,
            buffer_tracker: [[0 as u8; BUFFER_WIDTH]; BUFFER_HEIGHT],
        }
    }

    fn get_buffer_location(self) -> isize {
        (self.column + self.row * 80) as isize
    }

    fn new_line(&mut self) {
        self.row += 1;
        self.column = 0;
    }

    fn write_char_wrapper(&mut self, character: u8, cell_color: CellColor) {
        let char_location = self.get_buffer_location() * 2;
        let color_location = self.get_buffer_location() * 2 + 1;

        unsafe {
            *VGA_BUFFER.offset(char_location) = character;
            *VGA_BUFFER.offset(color_location) = cell_color.0;
        }

        // TODO self.buffer_tracker[self.column][self.row] = character;
    }

    pub fn print_byte_char(&mut self, cell: VgaCell) {
        if cell.character == b'\n' {
            self.new_line();
            return;
        }

        self.write_char_wrapper(cell.character, cell.color);

        if self.column >= BUFFER_WIDTH {
            self.new_line();
        }

        if self.row >= BUFFER_HEIGHT {
            // TODO
        }

        self.column += 1;
    }

    pub fn print_string(&mut self, string: &str, color: CellColor) {
        for ch in string.bytes() {
            match ch {
                0x20..=0x7e | b'\n' => self.print_byte_char(VgaCell::new(ch, color)),
                // write a invalid char if the char is not supported
                _ => self.print_byte_char(VgaCell::new(0xfe, color)),
            }
        }
    }
}

impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_string(s, CellColor::new(VgaColor::White, VgaColor::Black));
        Ok(())
    }
}

// reimplemented print macros
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    get_os_no_interrupt(|mut os| {
        os.printer.write_fmt(args).unwrap();
    });
}

test!("Basic VGA Print", || println!("Hello world!"));
test!("Long VGA Print", || println!(
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
));
test!("VGA Print new line", || println!("aaa\naaa\naaa"));
test!("Really long VGA Print", || {
    for _i in 0..=100 {
        println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    }
});
