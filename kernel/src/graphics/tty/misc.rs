use crate::graphics::tty::{Color, DEFAULT_FOREGROUND, EMPTY_BACKGROUND, Tty};

impl<'a> Tty<'a> {
    pub fn reset_color(&mut self) {
        self.current_background = EMPTY_BACKGROUND;
        self.current_foreground = DEFAULT_FOREGROUND;
        self.bold = false;
    }
}

pub fn calc_shadow_color(color: Color) -> Color {
    let (r, g, b) = color;

    (!r, !g, !b)
}
