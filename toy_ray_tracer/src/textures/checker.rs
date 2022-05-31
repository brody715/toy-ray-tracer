use crate::core::{SurfaceInteraction, Texture, TexturePtr};

pub struct CheckerTexture<T> {
    odd: TexturePtr<T>,
    even: TexturePtr<T>,
}

impl<T> CheckerTexture<T> {
    pub fn new(odd: TexturePtr<T>, even: TexturePtr<T>) -> Self {
        CheckerTexture { odd, even }
    }
}

impl<T> Texture<T> for CheckerTexture<T> {
    fn evaluate(&self, si: &SurfaceInteraction) -> T {
        let p = si.point;

        let sines = p[0].sin() * p[1].sin() * p[2].sin();
        if sines < 0.0 {
            self.odd.evaluate(si)
        } else {
            self.even.evaluate(si)
        }
    }
}
