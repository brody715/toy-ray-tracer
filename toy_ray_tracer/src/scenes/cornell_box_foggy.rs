use crate::camera::{Camera, CameraOpt};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::materials::{DiffuseLight, Lambertian};
use crate::scene::{RenderOptions, Scene};
use crate::shapes::{AARect, ConstantMedium, Cube, Plane};
use crate::textures::ConstantTexture;
use crate::transforms::Translate;
use crate::transforms::{Axis, Rotate};
use crate::vec::{vec3, Vec3};

fn create_world() -> Box<dyn Hittable> {
    let red = Lambertian::new(ConstantTexture::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(ConstantTexture::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(ConstantTexture::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(ConstantTexture::new(7.0, 7.0, 7.0));
    let mut world = HittableList::new();
    world.add(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, green));
    world.add(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.add(AARect::new(
        Plane::ZX,
        127.0,
        432.0,
        113.0,
        443.0,
        554.0,
        light,
    ));
    world.add(AARect::new(
        Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ));
    world.add(AARect::new(
        Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    ));
    world.add(AARect::new(
        Plane::XY,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ));
    let box1 = Translate::new(
        Rotate::new(
            Axis::Y,
            Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                white.clone(),
            ),
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    );
    let box2 = Translate::new(
        Rotate::new(
            Axis::Y,
            Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white,
            ),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    );
    world.add(ConstantMedium::new(
        box1,
        0.01,
        ConstantTexture::new(1.0, 1.0, 1.0),
    ));
    world.add(ConstantMedium::new(
        box2,
        0.01,
        ConstantTexture::new(0.0, 0.0, 0.0),
    ));
    Box::new(world)
}

pub fn create_scene(opt: RenderOptions) -> Scene {
    let camera = Camera::new(CameraOpt {
        look_from: Vec3::new(278.0, 278.0, -800.0),
        look_at: Vec3::new(278.0, 278.0, 0.0),
        view_up: vec3::YUP,
        vertical_fov: 40.0,
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
        String::from("cornell_box_foggy"),
        String::from("cornell box with fogs"),
    );
}
