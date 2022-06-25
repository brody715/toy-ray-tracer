use std::f32::consts::{FRAC_1_PI, PI};

use nalgebra::Matrix3;

use crate::{
    core::{
        sample,
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
        return self.albedo * FRAC_1_PI;
    }

    fn sample_wi(&self, _wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        sample::sample_hemisphere_cos_wi(normal)
    }

    fn sample_pdf(&self, wi: &Vec3f, _wo: &Vec3f, normal: &Vec3f) -> f32 {
        sample::sample_hemisphere_cos_pdf(normal, wi)
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
            let cos_reflected = reflected.dot(&normal);
            let cos_wo = wo.dot(&normal);

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

        let cos_wo = wo.dot(&normal);
        if same_hemisphere {
            return fresnel_dielectric(self.eta, cos_wo);
        } else {
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

    fn get_mixed_rf0(&self) -> Vec3f {
        let rf0 = eta_to_reflectivity(self.eta);
        let rf0 = vec3::lerp(&vec3::scalar(rf0), &self.base_color, self.metallic);
        rf0
    }
}

impl Bxdf for GltfPbrBxdf {
    fn is_delta(&self) -> bool {
        // self.roughness < f32::EPSILON
        false
    }

    fn f(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        if !vec3::is_same_hemisphere(wi, wo, normal) {
            return Vec3f::zeros();
        }

        let normal = get_up_normal(wo, normal);

        let cos_wi = normal.dot(wi);
        let cos_wo = normal.dot(&wo);
        let halfway = (wi + wo).normalize();
        let cos_half_wi = halfway.dot(wi);
        let cos_half = halfway.dot(&normal);

        let rf0 = self.get_mixed_rf0();

        let f1 = fresnel_schlick_v(&rf0, cos_wo);

        let f = fresnel_schlick_v(&rf0, cos_half_wi);

        // let f = fresnel_schlick_v(&rf0, halfway.dot(wo));
        // let f1 = f;

        let d = microfacet_distribution(self.roughness, cos_half);

        let g = microfacet_shadowing(self.roughness, &normal, &halfway, wo, wi);

        let c_diffuse = (1.0 - self.metallic) * self.base_color;

        // f_diffuse = (1 - F) * (1 / \pi) * c_diffuse
        let f_diffuse = vec3::elementwise_mult(&c_diffuse, &((vec3::scalar(1.0) - f1) * FRAC_1_PI));

        // f_specular = F * D * G / (4 * cos_o * cos_i)
        let f_specular = f * d * g / (4.0 * cos_wo * cos_wi);

        if log::max_level() >= log::Level::Trace {
            log::trace!(
                "f_diffuse: {:?}, base_color: {:?}, metallic: {:?}, f1: {:?}",
                f_diffuse,
                self.base_color,
                self.metallic,
                f1,
            );
        }

        return f_diffuse + f_specular;
    }

    fn sample_wi(&self, wo: &Vec3f, normal: &Vec3f) -> Vec3f {
        let normal = get_up_normal(wo, normal);

        let halfway = sample_microfacet(self.roughness, &normal);
        let wi = reflect(&wo, &halfway);
        if !vec3::is_same_hemisphere(&wi, wo, &normal) {
            return Vec3f::zeros();
        }
        return wi;
    }

    fn sample_pdf(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> f32 {
        // return sample::sample_hemisphere_cos_pdf(normal, wi);

        if !vec3::is_same_hemisphere(wi, wo, normal) {
            return 0.0;
        }

        let normal = get_up_normal(wo, normal);
        let halfway = (wi + wo).normalize();

        let rf0 = self.get_mixed_rf0();

        let cos_wo = normal.dot(wo);
        let cos_half = normal.dot(&halfway);
        let cos_half_wo = halfway.dot(&wo);

        let f = fresnel_schlick_v(&rf0, cos_wo).mean();

        let pdf = f * sample_microfacet_pdf(self.roughness, cos_half) / (4.0 * cos_half_wo.abs())
            + (1.0 - f) * sample::sample_hemisphere_cos_pdf(&normal, wi);

        return pdf;
    }
}

pub fn eta_to_reflectivity(eta: f32) -> f32 {
    ((1.0 - eta) / (1.0 + eta)).powi(2)
}

// $R_f(0) + (1 - R_f(0))(1 - \cos\theta)^5$
pub fn fresnel_schlick_v(rf_0: &Vec3f, cosine: f32) -> Vec3f {
    if *rf_0 == Vec3f::zeros() {
        return Vec3f::zeros();
    }

    let n = (1.0 - cosine.abs()).clamp(0.0, 1.0);
    return rf_0 + (vec3::scalar(1.0) - rf_0) * n.powi(5);
}

// dielectrics of the fresnel
pub fn fresnel_dielectric(eta: f32, cos_wo: f32) -> f32 {
    let sin2 = 1.0 - cos_wo * cos_wo;
    let eta2 = eta * eta;

    let cos2t = 1.0 - sin2 / eta2;
    if cos2t < 0.0 {
        return 1.0;
    }

    let t0 = cos2t.sqrt();
    let t1 = eta * t0;
    let t2 = eta * cos_wo;

    let rs = (cos_wo - t1) / (cos_wo + t1);
    let rp = (t0 - t2) / (t0 + t2);

    return (rs * rs + rp * rp) / 2.0;
}

// microfacet distribution evaluation
// @see http://graphicrants.blogspot.com/2013/08/specular-brdf-reference.html
// @param halfway (wi + wo).normalize()
// @param cos_wh halfway \cdot normal
pub fn microfacet_distribution(roughness: f32, cos_half: f32) -> f32 {
    if cos_half <= 0.0 {
        return 0.0;
    }

    let roughness2 = roughness * roughness;
    let cos_half2 = cos_half * cos_half;

    return roughness2 / (PI * (cos_half2 * (roughness2 - 1.0) + 1.0).powi(2));
}

pub fn microfacet_shadowing(
    roughness: f32,
    normal: &Vec3f,
    halfway: &Vec3f,
    wo: &Vec3f,
    wi: &Vec3f,
) -> f32 {
    let cos_wi = vec3::dot(wi, normal);
    let cos_wo = vec3::dot(wo, normal);
    let cos_half_wi = vec3::dot(halfway, wi);
    let cos_half_wo = vec3::dot(halfway, wo);

    return microfacet_shadowing1(roughness, cos_wi, cos_half_wi)
        * microfacet_shadowing1(roughness, cos_wo, cos_half_wo);
}

pub fn microfacet_shadowing1(roughness: f32, cos_d: f32, cos_half_d: f32) -> f32 {
    if cos_d * cos_half_d <= 0.0 {
        return 0.0;
    }

    let roughness2 = roughness * roughness;
    let cos_d2 = cos_d * cos_d;
    return 2.0 * cos_d.abs() / (cos_d.abs() + (roughness2 + (1.0 - roughness2) * cos_d2).sqrt());
}

pub fn sample_microfacet(roughness: f32, normal: &Vec3f) -> Vec3f {
    let rand_x = random::f32();
    let rand_y = random::f32();

    let phi = 2.0 * PI * rand_x;
    let theta = f32::atan(roughness * (rand_y / (1.0 - rand_y).sqrt()));
    let local_half_vector = Vec3f::new(
        phi.cos() * theta.sin(),
        phi.sin() * theta.sin(),
        theta.cos(),
    );

    (vec3::onb_fromz(normal) * local_half_vector).normalize()
}

pub fn sample_microfacet_pdf(roughness: f32, cos_half: f32) -> f32 {
    if cos_half < 0.0 {
        return 0.0;
    }
    return microfacet_distribution(roughness, cos_half) * cos_half;
}
