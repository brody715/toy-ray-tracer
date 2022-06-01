use std::f32::consts::FRAC_1_PI;

use crate::{
    core::{
        reflection::{
            eta_to_rf0, fresnel_dielectric, fresnel_schlick_v, microfacet_distribution,
            microfacet_shadowing, sample_hemisphere_cos_pdf, sample_microfacet,
            sample_microfacet_pdf,
        },
        vec3::{self, reflect},
        Bxdf, Spectrum, Vec3f,
    },
    utils::random,
};

fn get_up_normal(wo: &Vec3f, normal: &Vec3f) -> Vec3f {
    if normal.dot(wo) > 0.0 {
        *normal
    } else {
        -normal
    }
}

pub struct LambertianReflection {
    albedo: Spectrum,
}

impl LambertianReflection {
    pub fn new(albedo: Spectrum) -> Self {
        Self { albedo }
    }
}

impl Bxdf for LambertianReflection {
    fn is_delta(&self) -> bool {
        false
    }

    // $P_d / \pi$
    fn f(&self, _wi: &Vec3f, _wo: &Vec3f, _normal: &Vec3f) -> Vec3f {
        // TODO: Mul with 1 / \pi
        return self.albedo * FRAC_1_PI;
    }

    fn sample_wi(&self, _wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        let wi = vec3::random_hemisphere_cosine(normal);
        return wi;
    }

    fn sample_pdf(&self, wi: &Vec3f, _wo: &Vec3f, normal: &Vec3f) -> f32 {
        let cosine = wi.dot(&normal);
        let pdf = if cosine < 0.0 {
            0.0
        } else {
            cosine * FRAC_1_PI
        };
        pdf
    }
}

pub struct NaiveSpecularReflection {
    albedo: Spectrum,
    fuzz: f32,
}

impl NaiveSpecularReflection {
    pub fn new(albedo: Spectrum, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Bxdf for NaiveSpecularReflection {
    fn is_delta(&self) -> bool {
        true
    }

    fn f(&self, wi: &Vec3f, _wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        if wi.dot(normal) <= 0.0 {
            return Vec3f::zeros();
        }

        self.albedo
    }

    fn sample_wi(&self, wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        let mut reflected = vec3::reflect(&wo, normal);
        if self.fuzz > 0.0 {
            reflected += self.fuzz * vec3::random_in_unit_sphere();
        }
        reflected
    }

    fn sample_pdf(&self, _wi: &Vec3f, _wo: &Vec3f, _normal: &Vec3f) -> f32 {
        1.0
    }
}

pub struct TransparentTransmission {
    pub eta: f32,
    pub roughness: f32,
    pub color: Spectrum,
}

impl TransparentTransmission {
    pub fn new(eta: f32, roughness: f32, color: Spectrum) -> Self {
        Self {
            eta,
            roughness,
            color,
        }
    }
}

impl Bxdf for TransparentTransmission {
    fn is_delta(&self) -> bool {
        false
    }

    fn f(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        let same_hemisphere = vec3::is_same_hemisphere(wi, wo, normal);

        let normal = get_up_normal(wo, normal);

        if same_hemisphere {
            let halfway = (wi + wo).normalize();
            let cos_half = normal.dot(&halfway);
            let cos_wi = normal.dot(&wi);
            let cos_wo = normal.dot(&wo);
            let cos_half_wo = halfway.dot(&wo);

            let f = fresnel_dielectric(self.eta, cos_half_wo);
            let d = microfacet_distribution(self.roughness, cos_half);
            let g = microfacet_shadowing(self.roughness, &normal, &halfway, wo, wi);
            return vec3::scalar(1.0) * f * d * g / (4.0 * cos_wi * cos_wo).abs();
        } else {
            // TODO: check carefully
            let reflected = vec3::reflect(&-wi, &normal);
            let halfway = (reflected + wo).normalize();
            let cos_half = normal.dot(&halfway);
            let cos_half_wo = halfway.dot(&wo);
            let cos_reflected = reflected.dot(&wo);
            let cos_wo = normal.dot(&wo);

            let f = fresnel_dielectric(self.eta, cos_half_wo);
            let d = microfacet_distribution(self.roughness, cos_half);
            let g = microfacet_shadowing(self.roughness, &normal, &halfway, wo, &reflected);

            return self.color * (1.0 - f) * d * g / (4.0 * cos_wo * cos_reflected).abs();
        }
    }

    fn sample_wi(&self, wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        let normal = get_up_normal(wo, normal);
        let halfway = sample_microfacet(self.roughness, &normal);
        let cos_half_wo = wo.dot(&halfway);

        if random::f32() < fresnel_dielectric(self.eta, cos_half_wo) {
            let wi = reflect(wo, &halfway);
            if !vec3::is_same_hemisphere(&wi, wo, &normal) {
                return Vec3f::zeros();
            }
            return wi;
        } else {
            let reflected = reflect(wo, &halfway);
            let wi = -reflect(&reflected, &normal);
            if vec3::is_same_hemisphere(&wi, wo, &normal) {
                return Vec3f::zeros();
            }
            return wi;
        }
    }

    fn sample_pdf(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> f32 {
        let same_hemisphere = vec3::is_same_hemisphere(wi, wo, normal);

        let normal = get_up_normal(wo, normal);

        if same_hemisphere {
            let halfway = (wi + wo).normalize();

            let cos_half_wo = halfway.dot(&wo);
            let cos_half = halfway.dot(&normal);
            return fresnel_dielectric(self.eta, cos_half_wo)
                * sample_microfacet_pdf(self.roughness, cos_half)
                / (4.0 * cos_half_wo.abs());
        } else {
            let reflected = reflect(&-wi, &normal);
            let halfway = (reflected + wo).normalize();

            let cos_half_wo = halfway.dot(&wo);
            let cos_half = normal.dot(&halfway);

            let d = (1.0 - fresnel_dielectric(self.eta, cos_half_wo))
                * sample_microfacet_pdf(self.roughness, cos_half);
            return d / (4.0 * cos_half_wo.abs());
        }
    }
}

pub struct DeltaTransparentTransmission {
    eta: f32,
    albedo: Spectrum,
}

impl DeltaTransparentTransmission {
    pub fn new(eta: f32, albedo: Spectrum) -> Self {
        Self { eta, albedo }
    }
}

impl Bxdf for DeltaTransparentTransmission {
    fn is_delta(&self) -> bool {
        true
    }

    fn f(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        let same_hemisphere = vec3::is_same_hemisphere(wi, wo, normal);
        let normal = get_up_normal(wo, normal);

        if same_hemisphere {
            return vec3::scalar(1.0) * fresnel_dielectric(self.eta, wo.dot(&normal));
        } else {
            let cos_wo = normal.dot(wo);
            return self.albedo * (1.0 - fresnel_dielectric(self.eta, cos_wo));
        }
    }

    fn sample_wi(&self, wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        let normal = get_up_normal(wo, normal);
        let cos_wo = normal.dot(wo);
        if random::f32() < fresnel_dielectric(self.eta, cos_wo) {
            return vec3::reflect(wo, &normal);
        } else {
            return -wo;
        }
    }

    fn sample_pdf(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> f32 {
        let same_hemisphere = vec3::is_same_hemisphere(wi, wo, normal);
        let normal = get_up_normal(wo, normal);

        if same_hemisphere {
            return fresnel_dielectric(self.eta, wo.dot(&normal));
        } else {
            let cos_wo = normal.dot(wo);
            return 1.0 - fresnel_dielectric(self.eta, cos_wo);
        }
    }
}

pub struct GltfPbrBxdf {
    eta: f32,
    base_color: Spectrum,
    roughness: f32,
    metallic: f32,
}

impl GltfPbrBxdf {
    pub fn new(eta: f32, base_color: Spectrum, roughness: f32, metallic: f32) -> Self {
        Self {
            eta,
            base_color,
            roughness,
            metallic,
        }
    }

    fn get_rf0(&self) -> Vec3f {
        let rf0 = eta_to_rf0(self.eta);
        let rf0 = vec3::lerp(&vec3::scalar(rf0), &self.base_color, self.metallic);
        rf0
    }
}

impl GltfPbrBxdf {
    fn f_old(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        if !vec3::is_same_hemisphere(wi, wo, normal) {
            return Vec3f::zeros();
        }

        let normal = get_up_normal(wo, normal);

        let cos_wi = normal.dot(wi);
        let cos_wo = normal.dot(&wo);
        let halfway = (wi + wo).normalize();
        let cos_half_wi = halfway.dot(wi);
        let cos_half = halfway.dot(&normal);

        let rf0 = self.get_rf0();

        let f1 = fresnel_schlick_v(&rf0, cos_wo);

        let f = fresnel_schlick_v(&rf0, cos_half_wi);

        let d = microfacet_distribution(self.roughness, cos_half);

        let g = microfacet_shadowing(self.roughness, &normal, &halfway, wo, wi);

        let f_diffuse = vec3::elementwise_mult(
            &self.base_color,
            &((1.0 - self.metallic) * (vec3::scalar(1.0) - f1) * FRAC_1_PI),
        );

        let f_specular = f * d * g / (4.0 * cos_wo * cos_wi);

        return f_diffuse + f_specular;
    }
}

impl Bxdf for GltfPbrBxdf {
    fn is_delta(&self) -> bool {
        // self.roughness < f32::EPSILON
        false
    }

    fn f(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        self.f_old(wi, wo, normal)
    }

    fn sample_wi(&self, wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        let normal = get_up_normal(wo, normal);

        let rf0 = self.get_rf0();

        let cos_wo = normal.dot(wo);
        if random::f32() < fresnel_schlick_v(&rf0, cos_wo).mean() {
            let halfway = sample_microfacet(self.roughness, &normal);
            let wi = reflect(&wo, &halfway);
            if !vec3::is_same_hemisphere(&wi, wo, &normal) {
                return Vec3f::zeros();
            }
            return wi;
        } else {
            return vec3::random_hemisphere_cosine(&normal);
        }
    }

    fn sample_pdf(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> f32 {
        if !vec3::is_same_hemisphere(wi, wo, normal) {
            return 0.0;
        }

        let normal = get_up_normal(wo, normal);
        let halfway = (wi + wo).normalize();

        let rf0 = self.get_rf0();

        let cos_wo = normal.dot(wo);
        let cos_half = normal.dot(&halfway);
        let cos_half_wo = halfway.dot(&wo);

        let f = fresnel_schlick_v(&rf0, cos_wo).mean();

        return f * sample_microfacet_pdf(self.roughness, cos_half) / (4.0 * cos_half_wo.abs())
            + (1.0 - f) * sample_hemisphere_cos_pdf(&normal, wi);
    }
}
