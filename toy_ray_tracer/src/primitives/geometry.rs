use crate::core::{
    LightPtr, MaterialPtr, Primitive, Ray, ShapePtr, SurfaceInteraction, Transform, AABB,
};

pub struct GeometricPrimitive {
    pub shape: ShapePtr,
    pub transform: Transform,
    pub material: MaterialPtr,
    pub area_light: Option<LightPtr>,
}

impl GeometricPrimitive {
    pub fn new(
        shape: ShapePtr,
        transform: Transform,
        material: MaterialPtr,
        area_light: Option<LightPtr>,
    ) -> Self {
        Self {
            shape,
            transform,
            material,
            area_light,
        }
    }
}

impl Primitive for GeometricPrimitive {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<SurfaceInteraction> {
        let ray = self.transform.transform_ray(ray);

        let si = self.shape.intersect(&ray, t_min, t_max);

        let si = si.map(|mut si| {
            self.transform.transform_surface_iteraction(&mut si);

            si.material = Some(self.material.as_ref());
            si
        });

        si
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.shape
            .bounding_box(t0, t1)
            .map(|b| self.transform.transform_bounding_box(b))
    }

    fn pdf_value(&self, origin: &crate::core::Point3f, v: &crate::core::Vec3f) -> f32 {
        self.shape.pdf_value(origin, v)
    }

    fn random(&self, origin: &crate::core::Vec3f) -> crate::core::Vec3f {
        self.shape.random(origin)
    }
}
