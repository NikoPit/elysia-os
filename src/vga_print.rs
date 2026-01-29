const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

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
    row: u8,
    column: u8,
}

impl Printer {
    pub fn new() -> Self {
        Self { row: 1, column: 0 }
    }

    fn get_buffer_location(self) -> isize {
        (self.column * self.row) as isize
    }

    fn new_line(self) {}

    pub fn print_byte_char(&mut self, cell: VgaCell) {
        if cell.character == b'\n' {
            self.new_line();
            return;
        }

        let vga_buffer = 0xb8000 as *mut u8;
        unsafe {
            *vga_buffer.offset(self.get_buffer_location() * 2) = cell.character;
            *vga_buffer.offset(self.get_buffer_location() * 2 + 1) = cell.color.0;
        }

        if self.column >= BUFFER_WIDTH as u8 {
            self.column = 0;
            self.row += 1;
            self.new_line();
        }

        if self.row >= BUFFER_HEIGHT as u8 {
            // TODO
        }

        self.column += 1;
    }

    pub fn print_string(&mut self, string: &str, color: CellColor) {
        for ch in string.bytes() {
            match ch {
                0x20..=0x7e => self.print_byte_char(VgaCell::new(ch, color)),
                // write a invalid char if the char is not supported
                _ => self.print_byte_char(VgaCell::new(0xfe, color)),
            }
        }
    }
}
