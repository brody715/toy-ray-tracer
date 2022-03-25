use crate::camera::{Camera, CameraOpt};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::scene::{RenderOptions, Scene};
use crate::shapes::{AARect, Plane};
use crate::shapes::{ConstantMedium, BVH};
use crate::shapes::{Cube, MovingSphere, Sphere};
use crate::textures::{ConstantTexture, ImageTexture, NoiseTexture};
use crate::transforms::Translate;
use crate::transforms::{Axis, Rotate};
use crate::utils::random;
use crate::vec::{vec3, Vec3};

fn create_world() -> Box<dyn Hittable> {
    let white = Lambertian::new(ConstantTexture::new(0.73, 0.73, 0.73));
    let ground = Lambertian::new(ConstantTexture::new(0.48, 0.83, 0.53));
    let mut world = HittableList::new();
    let mut box_list1: Vec<Box<dyn Hittable>> = Vec::new();
    let nb = 20;
    for i in 0..nb {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (random::f32() + 0.01);
            let z1 = z0 + w;
            box_list1.push(Box::new(Cube::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    world.add(BVH::new(box_list1, 0.0, 1.0));
    let light = DiffuseLight::new(ConstantTexture::new(7.0, 7.0, 7.0));
    world.add(AARect::new(
        Plane::ZX,
        147.0,
        412.0,
        123.0,
        423.0,
        554.0,
        light,
    ));
    let center = Vec3::new(400.0, 400.0, 200.0);
    world.add(MovingSphere::new(
        center,
        center + Vec3::new(30.0, 0.0, 0.0),
        0.0,
        1.0,
        50.0,
        Lambertian::new(ConstantTexture::new(0.7, 0.3, 0.1)),
    ));
    world.add(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    ));
    world.add(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0),
    ));
    let boundary = Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    world.add(boundary.clone());
    world.add(ConstantMedium::new(
        boundary,
        0.2,
        ConstantTexture::new(0.2, 0.4, 0.9),
    ));
    let boundary = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    world.add(ConstantMedium::new(
        boundary,
        0.0001,
        ConstantTexture::new(1.0, 1.0, 1.0),
    ));
    let image = image::open("assets/earthmap.jpg")
        .expect("image not found")
        .to_rgb8();
    let (nx, ny) = image.dimensions();
    let data = image.into_raw();
    let texture = ImageTexture::new(data, nx, ny);
    world.add(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        Lambertian::new(texture),
    ));
    world.add(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(NoiseTexture::new(0.1)),
    ));
    let mut box_list2: Vec<Box<dyn Hittable>> = Vec::new();
    let ns = 10;
    for _ in 0..ns {
        box_list2.push(Box::new(Sphere::new(
            Vec3::new(
                165.0 * random::f32(),
                165.0 * random::f32(),
                165.0 * random::f32(),
            ),
            10.0,
            white.clone(),
        )));
    }
    world.add(Translate::new(
        Rotate::new(Axis::Y, BVH::new(box_list2, 0.0, 0.1), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    ));
    Box::new(world)
}

pub fn create_scene(opt: RenderOptions) -> Scene {
    let camera = Camera::new(CameraOpt {
        look_from: Vec3::new(478.0, 278.0, -600.0),
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
        Vec3::zeros(),
        String::from("scene2"),
        String::from("the next week final scene"),
    );
}
