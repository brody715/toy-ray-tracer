use crate::utils::random;

use crate::core::{
    light::LightTypeFlags, Light, LightPtr, LightType, Point3f, Ray, Spectrum, Vec3f,
};

pub struct LightList {
    lights: Vec<LightPtr>,
    area_lights: Vec<LightPtr>,
    inf_lights: Vec<LightPtr>,
}

impl From<Vec<LightPtr>> for LightList {
    fn from(lights: Vec<LightPtr>) -> Self {
        let mut list = LightList::new();

        for light in lights {
            list.add(light);
        }
        list
    }
}

impl LightList {
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            area_lights: Vec::new(),
            inf_lights: Vec::new(),
        }
    }

    pub fn add(&mut self, item: LightPtr) {
        let light_type = item.get_flags();
        if light_type.contains(LightType::Area) {
            self.area_lights.push(item.clone());
        }

        if light_type.contains(LightType::Infinite) {
            self.inf_lights.push(item.clone());
        }

        self.lights.push(item.clone());
    }
}

impl Light for LightList {
    fn background_l(&self, r: &Ray) -> Spectrum {
        // TODO: support multi environment light
        // TODO: Currently, we only accumulate them

        let mut radiance = Spectrum::zeros();

        for light in self.inf_lights.iter() {
            radiance += light.background_l(r);
        }

        radiance
    }

    fn get_flags(&self) -> LightTypeFlags {
        return LightType::List.into();
    }

    fn sample_wi(&self, point: &Point3f) -> Vec3f {
        if self.lights.is_empty() {
            return Vec3f::zeros();
        }

        let idx = random::usize(0..self.lights.len());
        let light = &self.lights[idx];

        light.sample_wi(point)
    }

    fn sample_pdf(&self, point: &Point3f, wi: &Vec3f) -> f32 {
        let mut pdf = 0.0;
        let lights = &self.lights;
        for light in lights.iter() {
            pdf += light.sample_pdf(point, wi);
        }

        pdf /= lights.len() as f32;
        pdf
    }
}
