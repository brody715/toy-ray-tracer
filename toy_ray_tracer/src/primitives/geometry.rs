use crate::core::{
    MaterialPtr, Point3f, Primitive, Ray, ShapePtr, SurfaceInteraction, Transform, Vec3f, AABB,
};

pub struct GeometricPrimitive {
    pub shape: ShapePtr,
    pub object_to_world: Transform,
    pub world_to_object: Transform,
    pub material: MaterialPtr,
}

impl GeometricPrimitive {
    pub fn new(shape: ShapePtr, object_to_world: Transform, material: MaterialPtr) -> Self {
        Self {
            shape,
            object_to_world: object_to_world.clone(),
            world_to_object: object_to_world.inverse(),
            material,
        }
    }
}

impl Primitive for GeometricPrimitive {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<SurfaceInteraction> {
        let ray = self.world_to_object.transform_ray(ray);

        let si = self.shape.intersect(&ray, t_min, t_max);

        let si = si.map(|mut si| {
            self.object_to_world.transform_surface_iteraction(&mut si);

            si.material = Some(self.material.as_ref());
            si
        });

        si
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.shape
            .bounding_box(t0, t1)
            .map(|b| self.object_to_world.transform_bounding_box(b))
    }

    fn sample_pdf(&self, point: &Point3f, wi: &Vec3f) -> f32 {
        let point = self.world_to_object.transform_point3(point);
        let wi = self.world_to_object.transform_unit_dir(wi);

        self.shape.sample_pdf(&point, &wi)
    }

    fn sample_wi(&self, point: &Vec3f) -> Vec3f {
        let point = self.world_to_object.transform_point3(point);
        let wi = self.shape.sample_wi(&point);

        self.object_to_world.transform_unit_dir(&wi)
    }
}
