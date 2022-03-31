function create_world() {
  const world = make_geometry_list([]);

  // ground
  world.push({
    kind: "sphere",
    center: [0, -1000, 0],
    radius: 1000.0,
    material: {
      kind: "lambertian",
      albedo: {
        kind: "checker_texture",
        even: {
          kind: "constant_texture",
          color: [0.2, 0.3, 0.1],
        },
        odd: {
          kind: "constant_texture",
          color: [0.9, 0.9, 0.9],
        },
      },
    },
  });

  world.push({
    kind: "transforms",
    params: [
      { kind: "rotate", axis: "Y", angle: -90 },
      { kind: "translate", offset: [0, 1, 0] },
    ],
    child: {
      // kind: "sphere",
      // center: [-4, 1, 0],
      // radius: 1,
      kind: "mesh",
      from_obj_file: {
        path: "",
      },
      from_obj: {
        file_path: "assets/models/cmu_cow/spot.obj",
      },
      material: {
        kind: "diffuse_light",
        emit: {
          kind: "image_texture",
          // color: [0.4, 0.2, 0.1],
          file_path: "assets/models/cmu_cow/spot_texture.png",
        },
      },
    },
  });
  return world;
}

const size = Utils.make_screen_size({
  width: 800,
  height: 800,
});

export default make_project({
  name: "mesh1",
  settings: {
    output_dir: "./output",
    width: size.width,
    height: size.height,
    nsamples: 10,
    max_depth: 15,
  },
  scene: {
    camera: {
      look_from: [13, 2, 3],
      look_at: [0, 0, 0],
      view_up: [0, 1, 0],
      vertical_fov: 20,
      aspect: size.aspect(),
      aperture: 0.0,
      focus_dist: 10.0,
      time0: 0.0,
      time1: 1.0,
    },
    sky: {
      kind: "solid",
      background: [0.7, 0.8, 1.0],
    },
    world: create_world(),
  },
});
