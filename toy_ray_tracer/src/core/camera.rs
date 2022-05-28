use crate::{core::vec3, core::Vec3, core::Ray, utils};
use std::f32;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    time0: f32,
    time1: f32,
    lens_radius: f32,
    opt: CameraOpt,
}

pub struct CameraOpt {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub view_up: Vec3,
    pub vertical_fov: f32,
    pub aspect: f32,
    pub aperture: f32,
    pub focus_dist: f32,
    pub time0: f32,
    pub time1: f32,
}

impl Camera {
    pub fn new(opt: CameraOpt) -> Self {
        let theta = opt.vertical_fov * f32::consts::PI / 180.0;
        let half_height = opt.focus_dist * f32::tan(theta / 2.0);
        let half_width = opt.aspect * half_height;
        let w = (opt.look_from - opt.look_at).normalize();
        let u = opt.view_up.cross(&w).normalize();
        let v = w.cross(&u);
        Camera {
            origin: opt.look_from,
            lower_left_corner: opt.look_from
                - half_width * u
                - half_height * v
                - opt.focus_dist * w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            u,
            v,
            time0: opt.time0,
            time1: opt.time1,
            lens_radius: opt.aperture / 2.0,
            opt,
        }
    }

    #[allow(dead_code)]
    pub fn set_aspect(&mut self, aspect: f32) {
        *self = Camera::new(CameraOpt { aspect, ..self.opt })
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let origin = if self.lens_radius == 0.0 {
            self.origin
        } else {
            let rd = self.lens_radius * vec3::random_in_unit_disk();
            let offset = self.u * rd.x + self.v * rd.y;
            self.origin + offset
        };
        let time = self.time0 + utils::random::f32() * (self.time1 - self.time0);
        Ray::new(
            origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - origin,
            time,
        )
    }
}
