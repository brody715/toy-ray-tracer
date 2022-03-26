use crate::camera::{Camera, CameraOpt};
use crate::hittable::Hittable;
use crate::materials::Lambertian;
use crate::scene::{RenderOptions, Scene};
use crate::shapes::Sphere;

use crate::textures::ImageTexture;
use crate::vec::{vec3, Vec3};
use std::sync::Arc;

fn create_world() -> Box<dyn Hittable> {
    let image = image::open("assets/earthmap.jpg")
        .expect(format!("{} image not found", "earthmap.jpg").as_str())
        .to_rgb8();
    let (nx, ny) = image.dimensions();
    let data = image.into_raw();
    let texture = Arc::new(ImageTexture::new(data, nx, ny));
    let earth = Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(texture)),
    );
    Box::new(earth)
}

pub fn create_scene(opt: RenderOptions) -> Scene {
    let camera = Camera::new(CameraOpt {
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::zeros(),
        view_up: vec3::YUP,
        vertical_fov: 20.0,
        aspect: opt.aspect(),
        aperture: 0.0,
        focus_dist: 10.0,
        time0: 0.0,
        time1: 1.0,
    });

    let world = create_world();

    return Scene::new(
        camera,
        world,
        Vec3::new(0.7, 0.8, 1.0),
        String::from("earth"),
        String::from("image texture earth"),
    );
}
