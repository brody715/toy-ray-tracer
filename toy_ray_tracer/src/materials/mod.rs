use crate::{
    hittable::HitRecord,
    material::Material,
    ray::Ray,
    texture::{Texture, TexturePtr},
    utils::random,
    vec::{vec3, Vec3},
};

pub struct Lambertian {
    albedo: TexturePtr,
}

impl Lambertian {
    pub fn new(albedo: TexturePtr) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit.p + hit.normal + vec3::random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p, ray.time());
        Some((scattered, self.albedo.value(hit.u, hit.v, &hit.p)))
    }

    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::zeros()
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut reflected = vec3::reflect(&ray.direction().normalize(), &hit.normal);
        if self.fuzz > 0.0 {
            reflected += self.fuzz * vec3::random_in_unit_sphere()
        };
        if reflected.dot(&hit.normal) > 0.0 {
            let scattered = Ray::new(hit.p, reflected, ray.time());
            Some((scattered, self.albedo))
        } else {
            None
        }
    }

    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::zeros()
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
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
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

        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let scattered = Ray::new(rec.p, direction, 0.0);
        return Some((scattered, attenuation));
    }

    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::zeros()
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
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, &p)
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
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let scattered = Ray::new(hit.p, vec3::random_in_unit_sphere(), ray.time());
        Some((scattered, self.albedo.value(hit.u, hit.v, &hit.p)))
    }

    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::zeros()
    }
}
