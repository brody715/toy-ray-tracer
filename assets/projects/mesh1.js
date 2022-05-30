function create_world() {
  const world = make_primitive_list([]);

  // ground
  world.push({
    kind: "geom",
    shape: {
      kind: "sphere",
      center: [0, -1000, 0],
      radius: 1000.0,
    },
    material: {
      kind: "lambertian",
      albedo: {
        kind: "checker_texture",
        even: {
          kind: "constant_texture",
          value: [0.2, 0.3, 0.1],
        },
        odd: {
          kind: "constant_texture",
          value: [0.9, 0.9, 0.9],
        },
      },
    },
  });

  world.push({
    kind: "geom",
    transforms: [
      {kind: "rotate", axis: [0, 1, 0], angle: -90},
      {kind: "translate", offset: [0, 1, 0]},
    ],
    shape: {
      kind: "uri",
      uri: "assets:///models/cmu_cow/spot.obj",
    },
    material: {
      kind: "lambertian",
      albedo: {
        kind: "image_texture",
        // kind: "constant_texture",
        // value: [0.4, 0.2, 0.1],
        uri: "assets:///models/cmu_cow/spot_texture.png",
      },
    },
  });

  world.push({
    kind: "geom",
    shape: {
      kind: "sphere",
      center: [0, 10, 0],
      radius: 1,
    },
    material: {
      kind: "diffuse_light",
      emit: [1.0, 1.0, 1.0],
    },
    area_light: {},
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
    max_depth: 4,
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
    environments: [
      {
        l: [0.7, 0.8, 1.0],
      },
    ],
    world: create_world(),
  },
});
