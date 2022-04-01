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

  const m_pink = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: Color.hex2vec3(0xed556a),
    },
  });

  world.push({
    kind: "transforms",
    params: [
      { kind: "rotate", axis: "Z", angle: -45},
      { kind: "translate", offset: [0, 0.8, 0] },
    ],
    child: {
      kind: "cylinder",
      center0: [0, 0, 0],
      center1: [0, 0.5, 0],
      radius: 0.8,
      material: m_pink,
    },
  });

  return world;
}

const size = Utils.make_screen_size({
  width: 800,
  height: 800,
});

export default make_project({
  name: "cylinder",
  settings: {
    output_dir: "./output",
    width: size.width,
    height: size.height,
    nsamples: 20,
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
    world: {
      kind: "list",
      children: create_world(),
      time0: 0.0,
      time1: 1.0,
    },
  },
});
