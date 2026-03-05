use vte::Perform;

use crate::{graphics::tty::Tty, s_println};

impl<'a> Perform for Tty<'a> {
    fn print(&mut self, c: char) {
        if self.cursor_x >= self.screen_width_char() as u32 {
            self.new_line();
        }

        if self.cursor_y >= self.screen_height_chars() as u32 {
            self.scroll_up();
        }

        self.push_char(c);
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => s_println!(
                "Unimplemented ansi escape code or something: {}",
                byte as char
            ),
        }
    }

    fn csi_dispatch(
        &mut self,
        _params: &vte::Params,
        _intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
        match action {
            _ => s_println!("Unimplemented csi dispatch asni escape code {}", action),
        }
    }
}
