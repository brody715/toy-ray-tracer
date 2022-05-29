pub mod noise;

use crate::{
    core::{Image, Vec3f},
    core::{Texture, TexturePtr},
};

#[derive(Clone)]
pub struct ConstantTexture {
    color: Vec3f,
}

impl ConstantTexture {
    pub fn new(color: Vec3f) -> Self {
        ConstantTexture { color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3f) -> Vec3f {
        self.color
    }
}

pub struct CheckerTexture {
    odd: TexturePtr,
    even: TexturePtr,
}

impl CheckerTexture {
    #[allow(dead_code)]
    pub fn new(odd: TexturePtr, even: TexturePtr) -> Self {
        CheckerTexture { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3f) -> Vec3f {
        let sines = f32::sin(10.0 * p.x) * f32::sin(10.0 * p.y) * f32::sin(10.0 * p.z);
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    image: Image,
}

impl ImageTexture {
    pub fn new(image: Image) -> Self {
        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Vec3f) -> Vec3f {
        let width = self.image.width() as usize;
        let height = self.image.height() as usize;
        let mut i = (u * width as f32) as usize;
        let mut j = ((1.0 - v) * height as f32) as usize;
        if i > width - 1 {
            i = width - 1
        }
        if j > height - 1 {
            j = height - 1
        }
        let pixel_idx = i + width * j;
        return self.image.get_pixel(pixel_idx);
    }
}
