use std::sync::Arc;

use crate::{core::Camera, core::PrimitivePtr};

use super::{
    light::{is_area_light, is_infinite_light},
    LightPtr,
};

pub struct Scene {
    pub(crate) camera: Arc<Camera>,
    pub(crate) world: PrimitivePtr,
    pub area_lights: Vec<LightPtr>,
    pub infinite_lights: Vec<LightPtr>,
}

impl Scene {
    pub fn new(camera: Arc<Camera>, world: PrimitivePtr, lights: Vec<LightPtr>) -> Self {
        let mut area_lights: Vec<LightPtr> = Vec::new();
        let mut infinite_lights: Vec<LightPtr> = Vec::new();

        for light in lights {
            if is_area_light(light.as_ref()) {
                area_lights.push(light);
            } else if is_infinite_light(light.as_ref()) {
                infinite_lights.push(light);
            } else {
                panic!("unknown light type {}", light.get_flags());
            }
        }

        Self {
            camera,
            world,
            area_lights,
            infinite_lights,
        }
    }
}
