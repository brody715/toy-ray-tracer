use crate::core::{vec3, Image, Point2f, Spectrum, Texture, TextureData, Vec3f};

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
        let mut j = (uv[1] * height as f32) as usize;
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

pub struct ImageTextureParams {
    pub scale: Spectrum,

    // tex_coords space has (0, 0) at lower left corner, so we need to flip face
    // but sometimes, the model loading library (such as easy-gltf) has already flipped the image
    pub flip: bool,
}

impl Default for ImageTextureParams {
    fn default() -> Self {
        Self {
            scale: Spectrum::new(1.0, 1.0, 1.0),
            flip: true,
        }
    }
}

impl ImageTexture<f32> {
    pub fn from_gray_image(gray: &image::GrayImage, params: ImageTextureParams) -> Self {
        let width = gray.width() as usize;
        let height = gray.height() as usize;
        let data = gray.pixels().map(|p| p[0] as f32).collect();
        Self::new(width, height, data, params)
    }
}

impl ImageTexture<Spectrum> {
    pub fn from_image(image: Image, params: ImageTextureParams) -> Self {
        let width = image.width() as usize;
        let height = image.height() as usize;
        let mut data = Vec::new();

        for i in 0..(width * height) {
            let value = image.get_pixel(i);
            data.push(value);
        }

        Self::new(width, height, data, params)
    }

    pub fn from_rgba_image(rgba: &image::RgbaImage, params: ImageTextureParams) -> Self {
        let width = rgba.width() as usize;
        let height = rgba.height() as usize;
        let data: Vec<Spectrum> = rgba
            .pixels()
            .map(|p| {
                let r = f32::from(p[0]) / 255.0;
                let g = f32::from(p[1]) / 255.0;
                let b = f32::from(p[2]) / 255.0;

                Vec3f::new(r, g, b)
            })
            .collect();

        Self::new(width, height, data, params)
    }

    pub fn from_rgb_image(rgb: &image::RgbImage, params: ImageTextureParams) -> Self {
        let width = rgb.width() as usize;
        let height = rgb.height() as usize;
        let data: Vec<Spectrum> = rgb
            .pixels()
            .map(|p| {
                let r = f32::from(p[0]) / 255.0;
                let g = f32::from(p[1]) / 255.0;
                let b = f32::from(p[2]) / 255.0;

                Vec3f::new(r, g, b)
            })
            .collect();

        Self::new(width, height, data, params)
    }
}

pub trait ImageTextureData {
    fn scale(&mut self, factor: &Spectrum);
}

impl ImageTextureData for f32 {
    fn scale(&mut self, factor: &Spectrum) {
        *self *= factor[0];
    }
}

impl ImageTextureData for Spectrum {
    fn scale(&mut self, factor: &Spectrum) {
        *self = vec3::elementwise_mult(self, factor);
    }
}

impl<T: Clone + ImageTextureData> ImageTexture<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>, params: ImageTextureParams) -> Self {
        let scale = params.scale;

        let mut data: Vec<T> = data
            .iter()
            .map(|v| {
                let mut v = v.clone();
                v.scale(&scale);
                v
            })
            .collect();

        // flip
        if params.flip {
            for y in 0..(height / 2) {
                for x in 0..width {
                    let o1 = y * width + x;
                    let o2 = (height - 1 - y) * width + x;
                    data.swap(o1, o2);
                }
            }
        }

        Self {
            mipmap: MipMap::new(width, height, data),
        }
    }
}

impl<T: TextureData> Texture<T> for ImageTexture<T> {
    fn evaluate(&self, si: &crate::core::SurfaceInteraction) -> T {
        self.mipmap.get_value(si.uv)
    }
}
