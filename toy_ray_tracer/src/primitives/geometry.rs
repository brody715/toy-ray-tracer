use crate::core::{
    MaterialPtr, Point3f, Primitive, Ray, ShapePtr, SurfaceInteraction, Vec3f, AABB,
};

pub struct GeometricPrimitive {
    pub shape: ShapePtr,
    pub material: MaterialPtr,
}

impl GeometricPrimitive {
    pub fn new(shape: ShapePtr, material: MaterialPtr) -> Self {
        Self { shape, material }
    }
}

impl Primitive for GeometricPrimitive {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<SurfaceInteraction> {
        let si = self.shape.intersect(&ray, t_min, t_max);

        let si = si.map(|mut si| {
            si.material = Some(self.material.as_ref());
            si
        });

        si
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.shape.bounding_box(t0, t1)
    }

    fn sample_pdf(&self, point: &Point3f, wi: &Vec3f) -> f32 {
        self.shape.sample_pdf(&point, &wi)
    }

    fn sample_wi(&self, point: &Vec3f) -> Vec3f {
        self.shape.sample_wi(&point)
    }
}
