use crate::{
    graphics::framebuffer::{Canvas, FRAME_BUFFER},
    s_println,
};

pub static WALLPAPER: &[u8] = include_bytes!("../../../resources/wallpaper.bin");

pub struct TTY {}

impl TTY {
    pub fn draw_wallpaper(&mut self) {
        let mut fb = FRAME_BUFFER.get().unwrap().lock();

        let width = 1280;
        let height = 720;
        let bpp = 4; // 每个像素占 4 字节 (BGRA)

        for y in 0..height {
            for x in 0..width {
                // 计算该像素在 bin 文件中的起始位置
                let i = (y * width + x) * bpp;

                // 从静态数组中读取颜色分量
                // 注意：由于我们转换时用了 BGRA，所以顺序是 B, G, R, A
                let b = WALLPAPER[i];
                let g = WALLPAPER[i + 1];
                let r = WALLPAPER[i + 2];

                // 第 4 位是 Alpha(i+3)，我们通常跳过它，或者用来做透明度计算

                // 调用你引以为傲的 write_pixel
                fb.write_pixel(x, y, r, g, b);
            }
        }
    }
}
