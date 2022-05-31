use std::f32::consts::PI;

use crate::{
    core::Ray,
    core::SurfaceInteraction,
    core::TexturePtr,
    core::{vec3, Color3, Vec3f},
    core::{Material, ScatterRecord},
    math::{pdfs::CosinePDF, ONB},
    utils::random,
};

pub struct NopMaterial;

impl Material for NopMaterial {
    fn scatter(&self, _ray: &Ray, _rec: &SurfaceInteraction) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, _ray: &Ray, _rec: &SurfaceInteraction, _scattered: &Ray) -> f32 {
        1.0
    }
}

pub struct Lambertian {
    albedo: TexturePtr,
}

impl Lambertian {
    pub fn new(albedo: TexturePtr) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &SurfaceInteraction) -> Option<ScatterRecord> {
        let attenuation = self.albedo.value(rec.uv[0], rec.uv[1], &rec.point);
        let pdf = Box::new(CosinePDF::from(rec.normal));

        Some(ScatterRecord {
            specular_ray: None,
            attenuation,
            pdf: Some(pdf),
        })
    }

    fn scattering_pdf(&self, _ray: &Ray, rec: &SurfaceInteraction, scattered: &Ray) -> f32 {
        let cosine = rec.normal.dot(&scattered.direction().normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

pub struct Metal {
    albedo: TexturePtr,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: TexturePtr, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &SurfaceInteraction) -> Option<ScatterRecord> {
        let mut reflected = vec3::reflect(&ray.direction().normalize(), &rec.normal);
        if self.fuzz > 0.0 {
            reflected += self.fuzz * vec3::random_in_unit_sphere()
        };
        let attenuation = self.albedo.value(rec.uv[0], rec.uv[1], &rec.point);
        let pdf = None;

        if reflected.dot(&rec.normal) > 0.0 {
            let specular_ray = Some(Ray::new(rec.point, reflected, ray.time()));

            return Some(ScatterRecord {
                specular_ray,
                attenuation,
                pdf,
            });
        } else {
            None
        }
    }

    fn scattering_pdf(&self, _ray: &Ray, _rec: &SurfaceInteraction, _scattered: &Ray) -> f32 {
        1.0
    }
}

pub struct Dielectric {
    // index of refraction
    ir: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ir: ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &SurfaceInteraction) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = &ray.direction().normalize();
        let cos_theta = vec3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || vec3::schlick(cos_theta, refraction_ratio) > random::f32() {
                vec3::reflect(&unit_direction, &rec.normal)
            } else {
                vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        let attenuation = Vec3f::new(1.0, 1.0, 1.0);
        let specular_ray = Some(Ray::new(rec.point, direction, ray.time()));
        return Some(ScatterRecord {
            specular_ray,
            attenuation,
            pdf: None,
        });
    }

    fn scattering_pdf(&self, _ray: &Ray, _rec: &SurfaceInteraction, _scattered: &Ray) -> f32 {
        1.0
    }
}

pub struct DiffuseLight {
    emit: TexturePtr,
}

impl DiffuseLight {
    pub fn new(emit: TexturePtr) -> Self {
        DiffuseLight { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _rec: &SurfaceInteraction) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, _ray: &Ray, _rec: &SurfaceInteraction, _scattered: &Ray) -> f32 {
        1.0
    }

    fn emitted(&self, _ray: &Ray, rec: &SurfaceInteraction) -> crate::core::Color3 {
        if rec.front_face {
            return self.emit.value(rec.uv[0], rec.uv[1], &rec.point);
        } else {
            return Color3::zeros();
        }
    }
}

#[derive(Clone)]
pub struct Isotropic {
    albedo: TexturePtr,
}

impl Isotropic {
    pub fn new(albedo: TexturePtr) -> Self {
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, rec: &SurfaceInteraction) -> Option<ScatterRecord> {
        let specular_ray = Some(Ray::new(
            rec.point,
            vec3::random_in_unit_sphere(),
            ray.time(),
        ));
        Some(ScatterRecord {
            specular_ray,
            attenuation: self.albedo.value(rec.uv[0], rec.uv[1], &rec.point),
            pdf: None,
        })
    }

    fn scattering_pdf(&self, _ray: &Ray, _rec: &SurfaceInteraction, _scattered: &Ray) -> f32 {
        1.0
    }
}
