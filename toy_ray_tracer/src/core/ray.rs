use crate::core::Vec3f;

#[derive(Debug, Default, Clone)]
pub struct Ray {
    a: Vec3f,
    b: Vec3f,
    time: f32,
}

impl Ray {
    pub fn new(a: Vec3f, b: Vec3f, time: f32) -> Self {
        Ray { a, b, time }
    }

    pub fn origin(&self) -> Vec3f {
        self.a
    }
    pub fn direction(&self) -> Vec3f {
        self.b
    }
    pub fn time(&self) -> f32 {
        self.time
    }
    pub fn point_at_parameter(&self, t: f32) -> Vec3f {
        self.a + t * self.b
    }
}
