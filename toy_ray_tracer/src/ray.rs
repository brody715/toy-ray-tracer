use crate::vec::Vec3;

#[derive(Default, Clone)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3, time: f32) -> Self {
        Ray { a, b, time }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }
    pub fn direction(&self) -> Vec3 {
        self.b
    }
    pub fn time(&self) -> f32 {
        self.time
    }
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + t * self.b
    }
}
