use bootloader_api::info::PixelFormat;
use conquer_once::spin::OnceCell;
use spin::Mutex;

pub static FRAME_BUFFER: OnceCell<Mutex<FrameBuffer>> = OnceCell::uninit();

pub struct FrameBuffer {
    buffer: &'static mut [u8],
    info: bootloader_api::info::FrameBufferInfo,
}

impl FrameBuffer {
    pub fn new(frame_buffer: &'static mut bootloader_api::info::FrameBuffer) -> Self {
        let info = frame_buffer.info();
        let buffer = frame_buffer.buffer_mut();

        // Clear screen
        buffer.fill(0);

        Self { info, buffer }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        // Offset of the pixel from the start
        // of the framebuffer (in pixels)
        let pixels_offset = (y * self.info.stride) + x;
        // Offset in bytes
        let bytes_offset = pixels_offset * self.info.bytes_per_pixel;

        match self.info.pixel_format {
            PixelFormat::Rgb => {
                self.buffer[bytes_offset] = r;
                self.buffer[bytes_offset + 1] = g;
                self.buffer[bytes_offset + 2] = b;
            }
            PixelFormat::Bgr => {
                self.buffer[bytes_offset] = b;
                self.buffer[bytes_offset + 1] = g;
                self.buffer[bytes_offset + 2] = r;
            }
            _ => {
                panic!("Unsupported pixel format. Possible old hardware");
            }
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }
}
