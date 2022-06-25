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
      // albedo: [0.2, 0.3, 0.1],
      // albedo: [0.9, 0.9, 0.9],
      albedo: {
        kind: "checker_texture",
        even: [0.2, 0.3, 0.1],
        odd: [0.9, 0.9, 0.9],
      },
    },
  });
  // world.pop();

  const color_pink = Color.hex2vec3(0xed556a);
  const color_black = [0, 0, 0];

  const m_pink = make_material({
    kind: "lambertian",
    albedo: color_pink,
  });

  const m_pink_metal = make_material({
    kind: "metal",
    albedo: color_pink,
    fuzz: 0,
  });

  const m_pink_transparent = make_material({
    kind: "transparent",
    albedo: color_pink,
    eta: 1.5,
    roughness: 0.2,
  });

  const m_silver_gltf_pbr = make_material({
    kind: "gltf_pbr",
    // base_color: Color.hex2vec3(0xc0c0c0),
    base_color: color_pink,
    // base_color: [1.0, 0.766, 0.336],
    metallic: 1.0,
    roughness: 0.0,
    eta: 1.5,
  });

  // Light
  world.push({
    kind: "geom",
    shape: {
      kind: "sphere",
      center: [0, 5, 5],
      radius: 1,
    },
    material: {
      kind: "diffuse_light",
      emit: Vec3.mul([1, 1, 1], 10),
    },
    area_light: {},
  });
  // world.pop();

  world.push({
    kind: "geom",
    transforms: [
      {kind: "rotate", axis: [0, 1, 0], angle: 90},
      {
        kind: "translate",
        offset: [0, 0, 0],
      },
    ],
    shape: {
      kind: "sphere",
      center: [0, 1, 0],
      radius: 1,
    },
    material: m_silver_gltf_pbr,
  });
  // world.pop();

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
    // nsamples: 20,
    nsamples: 200,
    max_depth: 20,
    mis_weight: 0.5,
    // mis_weight: 1.0,
  },
  scenes: [
    {
      kind: "custom",
      camera: {
        look_from: [0, 2, 15],
        look_at: [0, 2, 0],
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
          l: Vec3.mul([0.7, 0.8, 1], 0.0),
        },
      ],
      world: create_world(),
    },
  ],
});
