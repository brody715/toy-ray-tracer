use crate::core::{SurfaceInteraction, Texture, TextureData, TexturePtr};

pub struct CheckerTexture<T> {
    odd: TexturePtr<T>,
    even: TexturePtr<T>,
}

impl<T: TextureData> CheckerTexture<T> {
    pub fn new(odd: TexturePtr<T>, even: TexturePtr<T>) -> Self {
        CheckerTexture { odd, even }
    }
}

impl<T: TextureData> Texture<T> for CheckerTexture<T> {
    fn evaluate(&self, si: &SurfaceInteraction) -> T {
        let p = si.point;

        let sines = f32::sin(10.0 * p[0]) * f32::sin(10.0 * p[1]) * f32::sin(10.0 * p[2]);
        if sines < 0.0 {
            self.odd.evaluate(si)
        } else {
            self.even.evaluate(si)
        }
    }
}
