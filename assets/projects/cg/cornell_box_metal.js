export function create_world() {
  const m_red = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      value: [0.65, 0.05, 0.05],
    },
  });

  const m_white = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      value: [0.73, 0.73, 0.73],
    },
  });

  const m_green = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      value: [0.12, 0.45, 0.15],
    },
  });

  const m_light = make_material({
    kind: "diffuse_light",
    emit: {
      kind: "constant_texture",
      value: [8.0, 8.0, 8.0],
    },
  });

  const m_aluminu = make_material({
    kind: "metal",
    albedo: {
      kind: "constant_texture",
      value: [0.8, 0.85, 0.88],
    },
    fuzz: 0.0,
  });

  const m_transparent = make_material({
    // kind: "dielectric",
    kind: "transparent",
    albedo: [1, 1, 1],
    // albedo: Color.hex2vec3(0xed556a),
    eta: 1.5,
    roughness: 0.1,
  });

  const m_glass = make_material({
    kind: "dielectric",
    ir: 1.5,
  });

  const world = make_primitive_list([]);

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [555, 0, 0],
      v1: [555, 555, 555],
    },
    material: m_green,
  });

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, 0, 0],
      v1: [0, 555, 555],
    },
    material: m_red,
  });

  // const lights = make_primitive({
  //   kind: "list",
  //   children: [
  //     {
  //       kind: "flip_face",
  //       child: {
  //         kind: "rect",
  //         v0: [213, 554, 227],
  //         v1: [343, 554, 332],
  //         // radius: 100,
  //         material: m_light,
  //       },
  //     },
  //     {
  //       kind: "sphere",
  //       center: [190, 90, 190],
  //       radius: 90,
  //       material: m_light,
  //     },
  //   ],
  // });

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [213, 554, 227],
      v1: [343, 554, 332],
      // radius: 100,
    },
    material: m_light,
    area_light: {},
  });

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, 555, 0],
      v1: [555, 555, 555],
    },
    material: m_white,
  });

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, 0, 0],
      v1: [555, 0, 555],
    },
    material: m_white,
  });

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, 0, 555],
      v1: [555, 555, 555],
    },
    material: m_white,
  });

  // cubes
  let cube1 = make_primitive({
    kind: "geom",
    transforms: [
      {kind: "rotate", axis: [0, 1, 0], angle: 15},
      {kind: "translate", offset: [265, 0, 295]},
    ],
    shape: {
      kind: "cube",
      p_min: [0, 0, 0],
      p_max: [165, 330, 165],
    },
    material: m_aluminu,
  });

  let cube2 = make_primitive({
    kind: "geom",
    shape: {
      kind: "sphere",
      center: [190, 90, 190],
      radius: 90,
    },
    // material: m_transparent,
    material: m_glass,
  });

  // let cube2 = make_geometry({
  //   kind: "translate",
  //   offset: [265, 0, 295],
  //   child: {
  //     kind: "rotate",
  //     axis: "Y",
  //     angle: 15.0,
  //     child: {
  //       kind: "cube",
  //       p_min: [0, 0, 0],
  //       p_max: [165, 330, 165],
  //       material: m_white,
  //     },
  //   },
  // });

  world.push(cube1, cube2);

  return world;
}

export default make_project({
  name: "cornell_box_metal",
  settings: {
    output_dir: "./output/cg",
    height: 500,
    width: 500,
    // nsamples: 1000,
    nsamples: 1000,
    // nsamples: 3,
    max_depth: 50,
  },
  scenes: [
    {
      kind: "custom",
      camera: {
        look_from: [278, 278, -800],
        look_at: [278, 278, 0],
        view_up: [0, 1, 0],
        vertical_fov: 40.0,
        aspect: 1.0,
        aperture: 0.0,
        focus_dist: 10.0,
        time0: 0.0,
        time1: 0.0,
      },
      world: create_world(),
    },
  ],
});
