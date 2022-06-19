pub mod sample {
    use std::f32::consts::FRAC_1_PI;

    use crate::core::{vec3, Vec3f};

    pub fn sample_hemisphere_cos_wi(normal: &Vec3f) -> Vec3f {
        let wi = vec3::random_cosine_direction();
        if wi.dot(normal) > 0.0 {
            wi
        } else {
            -wi
        }
    }

    pub fn sample_hemisphere_cos_pdf(normal: &Vec3f, wi: &Vec3f) -> f32 {
        let cosine = normal.dot(wi);
        if cosine < 0.0 {
            0.0
        } else {
            cosine * FRAC_1_PI
        }
    }
}
