use crate::core::{Color3, Image, Point2f, Spectrum, Texture, TextureData};

struct MipMap<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> MipMap<T> {
    fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        MipMap {
            data,
            width,
            height,
        }
    }

    fn get_value(&self, uv: Point2f) -> T {
        let width = self.width as usize;
        let height = self.height as usize;
        let mut i = (uv[0] * width as f32) as usize;
        let mut j = ((1.0 - uv[1]) * height as f32) as usize;
        if i > width - 1 {
            i = width - 1
        }
        if j > height - 1 {
            j = height - 1
        }
        let pixel_idx = i + width * j;
        return self.data[pixel_idx].clone();
    }
}

pub struct ImageTexture<T> {
    mipmap: MipMap<T>,
}

impl ImageTexture<Spectrum> {
    pub fn from_image(image: Image) -> Self {
        Self::from_image_convert(image, |s| s.clone())
    }
}

impl<T: Clone> ImageTexture<T> {
    pub fn from_image_convert<F: Fn(&Color3) -> T>(image: Image, convert: F) -> Self {
        let width = image.width() as usize;
        let height = image.height() as usize;
        let mut data = Vec::new();

        for i in 0..(width * height) {
            let pixel_idx = i;
            let value = convert(&image.get_pixel(pixel_idx));
            data.push(value);
        }
        let mipmap = MipMap::new(width, height, data);
        Self { mipmap }
    }
}

impl<T: TextureData> Texture<T> for ImageTexture<T> {
    fn evaluate(&self, si: &crate::core::SurfaceInteraction) -> T {
        self.mipmap.get_value(si.uv)
    }
}
