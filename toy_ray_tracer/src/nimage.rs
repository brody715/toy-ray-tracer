use std::path::Path;

use crate::vec::Color3;
use image::{save_buffer_with_format, ImageResult};

pub struct Image {
    width: usize,
    height: usize,
    buf: Vec<u8>,
}

impl Image {
    const CHANNEL_NUMS: usize = 3;

    pub fn new(width: usize, height: usize) -> Image {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(width * height * Self::CHANNEL_NUMS, 0);
        Image { width, height, buf }
    }

    pub fn set_pixel(&mut self, idx: usize, color: Color3) {
        const FMAX_COLOR: f32 = 255.9;
        // channel_nums
        let start_idx = idx * Self::CHANNEL_NUMS;

        for i in 0..3 {
            self.buf[start_idx + i] = (color[i] * FMAX_COLOR) as u8
        }
    }

    #[allow(dead_code)]
    pub fn slice(&self, s: usize, e: usize) -> &[u8] {
        return &self.buf[s..e];
    }

    pub fn save_to_png<P: AsRef<Path>>(&self, path: P) -> ImageResult<()> {
        save_buffer_with_format(
            path,
            &self.buf,
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgb8,
            image::ImageFormat::Png,
        )
    }
}
