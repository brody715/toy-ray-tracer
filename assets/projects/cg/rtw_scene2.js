function create_world() {
  const world = make_primitive_list([]);

  const m_white = make_material({
    kind: "lambertian",
    albedo: [0.73, 0.73, 0.73],
  });

  const m_ground = make_material({
    kind: "lambertian",
    albedo: [0.48, 0.83, 0.53],
  });

  const l = 7.0;
  const m_light = make_material({
    kind: "diffuse_light",
    emit: [l, l, l],
  });

  // add cubes
  const n = 20;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      let w = 100;
      let x0 = -1000 + i * w;
      let z0 = -1000 + j * w;
      let y0 = 0;
      let x1 = x0 + w;
      let y1 = 100 * (Vec3.random_f32() + 0.01);
      let z1 = z0 + w;

      world.push({
        kind: "geom",
        shape: {
          kind: "cube",
          p_min: [x0, y0, z0],
          p_max: [x1, y1, z1],
        },
        material: m_ground,
      });
    }
  }

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [123, 554, 147],
      v1: [423, 554, 412],
    },
    material: m_light,
    area_light: {},
  });

  world.push({
    kind: "geom",
    shape: {
      kind: "sphere",
      center: [400, 400, 200],
      radius: 50,
    },
    material: {
      kind: "lambertian",
      albedo: [0.7, 0.3, 0.1],
    },
  });

  world.push({
    kind: "geom",
    shape: {kind: "sphere", center: [260, 150, 45], radius: 50},
    material: {kind: "dielectric", ir: 1.5},
  });

  world.push({
    kind: "geom",
    shape: {kind: "sphere", center: [0, 150, 145], radius: 50},
    material: {kind: "metal", albedo: [0.8, 0.8, 0.9], fuzz: 1.0},
  });

  // near earth
  world.push({
    kind: "geom",
    shape: {
      kind: "sphere",
      center: [360, 150, 145],
      radius: 70,
    },
    material: {
      //   kind: "dielectric",
      //   ir: 1.5,
      kind: "metal",
      albedo: [0.2, 0.4, 0.9],
      fuzz: 0.4,
    },
  });

  // s2 medium
  world.push({
    kind: "geom",
    shape: {
      kind: "sphere",
      center: [0, 0, 0],
      radius: 5000,
    },
    material: {
      kind: "dielectric",
      ir: 1.5,
    },
  });

  // earth
  world.push({
    kind: "geom",
    shape: {
      kind: "sphere",
      center: [400, 200, 400],
      radius: 100,
    },
    material: {
      kind: "lambertian",
      albedo: {
        kind: "image_texture",
        uri: "assets:///textures/earthmap.jpg",
      },
    },
  });

  world.push({
    kind: "geom",
    shape: {
      kind: "sphere",
      center: [220, 280, 300],
      radius: 80,
    },
    material: {kind: "metal", albedo: [0.8, 0.8, 0.9], fuzz: 0.4},
  });

  // small boxes
  const ns = 1000;
  const boxes = make_primitive_list([]);
  for (let i = 0; i < ns; i++) {
    boxes.push({
      kind: "geom",
      shape: {
        kind: "sphere",
        center: [
          165 * Vec3.random_f32(),
          165 * Vec3.random_f32(),
          165 * Vec3.random_f32(),
        ],
        radius: 10,
      },
      material: m_white,
    });
  }

  world.push({
    kind: "container",
    transforms: [
      {kind: "rotate", axis: [0, 1, 0], angle: 15},
      {
        kind: "translate",
        offset: [-100, 270, 395],
      },
    ],
    children: boxes,
  });

  return world;
}

const size = {
  width: 800,
  height: 800,
  aspect: function () {
    return this.width / this.height;
  },
};

export default make_project({
  name: "rtw_scene2",
  settings: {
    output_dir: "./output/cg",
    width: size.width,
    height: size.height,
    nsamples: 1000,
    max_depth: 40,
    mis_weight: 0.5,
  },
  scenes: [
    {
      kind: "custom",
      camera: {
        look_from: [478, 278, -600],
        look_at: [278, 278, 0],
        view_up: [0, 1, 0],
        vertical_fov: 40,
        aspect: size.aspect(),
        aperture: 0.0,
        focus_dist: 10.0,
        time0: 0.0,
        time1: 1.0,
      },
      world: create_world(),
    },
  ],
});
