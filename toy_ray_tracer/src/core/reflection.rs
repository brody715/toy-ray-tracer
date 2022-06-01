use nalgebra::Matrix3;

use super::{vec3, Spectrum, Vec3f};
use crate::utils::random;
use std::f32::consts::{FRAC_1_PI, PI};

pub struct Bsdf {
    normal: Vec3f,
    bxdf: BxdfPtr,
}

impl Bsdf {
    pub fn new(normal: Vec3f) -> Self {
        Self {
            normal,
            bxdf: Box::new(NopBxdf {}),
        }
    }

    pub fn set(&mut self, bxdf: BxdfPtr) {
        self.bxdf = bxdf
    }

    pub fn set_raw<T: Bxdf + 'static>(&mut self, bxdf: T) {
        self.bxdf = Box::new(bxdf);
    }
}

impl Bsdf {
    // delta distribution, like specular
    #[inline]
    pub fn is_delta(&self) -> bool {
        self.bxdf.is_delta()
    }

    // f_r(w_i, w_o) -> value, brdf function
    #[inline]
    #[allow(dead_code)]
    fn f(&self, wi: &Vec3f, wo: &Vec3f) -> Spectrum {
        self.bxdf.f(wi, wo, &self.normal)
    }

    // f_r(w_i, w_o) * \cos\theta, brdf with cos attenuation
    #[inline]
    pub fn f_cos(&self, wi: &Vec3f, wo: &Vec3f) -> Spectrum {
        self.bxdf.f(wi, wo, &self.normal) * self.cosine(wi)
    }

    #[inline]
    fn cosine(&self, wi: &Vec3f) -> f32 {
        f32::abs(wi.dot(&self.normal))
    }

    // importance sampling
    #[inline]
    pub fn sample_wi(&self, wo: &Vec3f) -> Vec3f {
        self.bxdf.sample_wi(wo, &self.normal)
    }

    #[inline]
    pub fn sample_pdf(&self, wi: &Vec3f, wo: &Vec3f) -> f32 {
        self.bxdf.sample_pdf(wi, wo, &self.normal)
    }
}

pub trait Bxdf {
    // delta distribution, like specular
    fn is_delta(&self) -> bool;

    // f_r(w_i, w_o) -> value, brdf function
    fn f(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> Vec3f;

    // importance sampling
    fn sample_wi(&self, wo: &Vec3f, normal: &Vec3f) -> Vec3f;

    fn sample_pdf(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> f32;
}

pub type BxdfPtr = Box<dyn Bxdf>;

pub struct NopBxdf {}
impl Bxdf for NopBxdf {
    fn is_delta(&self) -> bool {
        todo!()
    }

    fn f(&self, _wi: &Vec3f, _wo: &Vec3f, _normal: &Vec3f) -> Vec3f {
        todo!()
    }

    fn sample_wi(&self, _wo: &Vec3f, _normal: &Vec3f) -> Vec3f {
        todo!()
    }

    fn sample_pdf(&self, _wi: &Vec3f, _wo: &Vec3f, _normal: &Vec3f) -> f32 {
        todo!()
    }
}

pub fn eta_to_rf0(eta: f32) -> f32 {
    ((1.0 - eta) / (1.0 + eta)).powi(2)
}

// $R_f(0) + (1 - R_f(0))(1 - \cos\theta)^5$
pub fn fresnel_schlick(rf_0: f32, cosine: f32) -> f32 {
    if rf_0 == 0.0 {
        return 0.0;
    }

    let n = (1.0 - cosine.abs()).clamp(0.0, 1.0);
    return rf_0 + (1.0 - rf_0) * n.powi(5);
}

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

pub fn fresnel_conductor(eta_i: &Vec3f, eta_t: &Vec3f, cos_wo: f32) -> Vec3f {
    if cos_wo < 0.0 {
        return Vec3f::zeros();
    }

    let cos_wo = cos_wo.clamp(-1.0, 1.0);
    let cos_wo2 = cos_wo * cos_wo;
    let cos_wo2v = vec3::scalar(cos_wo2);
    let sin_wo2 = (1.0 - cos_wo2).clamp(0.0, 1.0);
    let eta_i2 = vec3::elementwise_mult(eta_i, eta_i);
    let eta_t2 = vec3::elementwise_mult(eta_t, eta_t);

    let t0 = eta_i2 - eta_t2 - cos_wo2v;
    let a2_plus_b2 = vec3::sqrt(
        vec3::elementwise_mult(&t0, &t0) + 4.0 * vec3::elementwise_mult(&eta_i2, &eta_t2),
    );
    let t1 = a2_plus_b2 + cos_wo2v;

    let a = vec3::sqrt((a2_plus_b2 + t0) / 2.0);
    let t2 = 2.0 * a * cos_wo;
    let rs = vec3::elementwise_div(&(t1 - t2), &(t1 + t2));

    let t3 = cos_wo2 * a2_plus_b2 + vec3::scalar(sin_wo2 * sin_wo2);
    let t4 = t2 * sin_wo2;
    let rp = vec3::elementwise_mult(&rs, &vec3::elementwise_div(&(t3 - t4), &(t3 + t4)));

    return (rp + rs) / 2.0;
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

pub fn microfacet_shadowing_v(
    roughness: f32,
    cos_wi: f32,
    cos_wo: f32,
    cos_half_wi: f32,
    cos_half_wo: f32,
) -> f32 {
    return microfacet_shadowing1(roughness, cos_wi, cos_half_wi)
        * microfacet_shadowing1(roughness, cos_wo, cos_half_wo);
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

    (basis_fromz(normal) * local_half_vector).normalize()
}

pub fn sample_microfacet_pdf(roughness: f32, cos_half: f32) -> f32 {
    if cos_half < 0.0 {
        return 0.0;
    }
    return microfacet_distribution(roughness, cos_half) * cos_half;
}

// Constructs a basis from a direction
pub fn basis_fromz(v: &Vec3f) -> Matrix3<f32> {
    // https://graphics.pixar.com/library/OrthonormalB/paper.pdf
    let z = v.normalize();
    let sign = 1.0_f32.copysign(z[2]);
    let a = -1.0 / (sign + z[2]);
    let b = z[0] * z[1] * a;
    let x = Vec3f::new(1.0 + sign * z[0] * z[0] * a, sign * b, -sign * z[0]);
    let y = Vec3f::new(b, sign + z[1] * z[1] * a, -z[1]);
    return Matrix3::from_columns(&[x, y, z]);
}

pub fn sample_hemisphere_cos_pdf(normal: &Vec3f, direction: &Vec3f) -> f32 {
    let cosine = normal.dot(direction);
    if cosine < 0.0 {
        0.0
    } else {
        cosine * FRAC_1_PI
    }
}
