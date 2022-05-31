use super::{Material, Point2f, Point3f, Vec3f};

pub struct SurfaceInteraction<'a> {
    pub t_hit: f32,
    pub point: Point3f,
    pub uv: Point2f,
    // 射出光线（ Ray 反方向）
    pub wo: Vec3f,

    pub normal: Vec3f,
    pub front_face: bool,

    pub material: Option<&'a dyn Material>,
}

impl<'a> SurfaceInteraction<'a> {
    pub fn new(t_hit: f32, p: Point3f, uv: Point2f, wo: Vec3f, normal: Vec3f) -> Self {
        let front_face = wo.dot(&normal) > 0.0;
        let normal = if front_face { normal } else { -normal };

        Self {
            t_hit,
            point: p,
            uv,
            wo: wo.normalize(),
            normal,
            front_face,
            material: None,
            // bsdf: None,
            // emitted: Color3::zeros(),
        }
    }

    pub fn flip_normal(&mut self) -> &mut Self {
        self.front_face = !self.front_face;
        self.normal = -self.normal;
        return self;
    }
}
