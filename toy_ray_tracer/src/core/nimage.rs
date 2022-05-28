use std::path::Path;

use crate::core::Color3;
use anyhow::Context;
use image::{save_buffer_with_format, ImageBuffer, ImageResult, Rgb};

#[derive(Clone)]
pub struct Image {
    width: usize,
    height: usize,
    buf: Vec<u8>,
}

impl From<ImageBuffer<Rgb<u8>, Vec<u8>>> for Image {
    fn from(image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self {
        let (width, height) = image.dimensions();
        let buf: Vec<u8> = image.into_raw();
        Self {
            width: width as usize,
            height: height as usize,
            buf,
        }
    }
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
            let mut cv = color[i];
            if cv.is_nan() {
                cv = 0.0
            }
            self.buf[start_idx + i] = (cv.clamp(0.0, 0.9999) * FMAX_COLOR) as u8
        }
    }

    pub fn get_pixel(&self, pixel_idx: usize) -> Color3 {
        let idx = pixel_idx * Self::CHANNEL_NUMS;

        let r = self.buf[idx] as f32 / 255.0;
        let g = self.buf[idx + 1] as f32 / 255.0;
        let b = self.buf[idx + 2] as f32 / 255.0;
        Color3::new(r, g, b)
    }

    #[allow(dead_code)]
    pub fn slice(&self, s: usize, e: usize) -> &[u8] {
        return &self.buf[s..e];
    }

    pub fn load_png<P: AsRef<Path>>(path: P) -> anyhow::Result<Image> {
        let image = image::open(path).context("failed to load image")?.to_rgb8();
        Ok(image.into())
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

    /// Get the image's width.
    #[inline]
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the image's height.
    #[inline]
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }
}
