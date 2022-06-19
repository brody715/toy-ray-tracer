export function create_world() {
  const red = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      value: [0.65, 0.05, 0.05],
    },
  });

  const white = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      value: [0.73, 0.73, 0.73],
    },
  });

  const green = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      value: [0.12, 0.45, 0.15],
    },
  });

  const light = make_material({
    kind: "diffuse_light",
    emit: {
      kind: "constant_texture",
      value: Vec3.mul([7.0, 7.0, 7.0], 7.0 / 7.0),
    },
  });

  const world = make_primitive_list([]);

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [555, 0, 0],
      v1: [555, 555, 555],
    },
    material: green,
  });

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, 0, 0],
      v1: [0, 555, 555],
    },
    material: red,
  });

  // world.push({
  //   kind: "geom",
  //   flip_face: true,
  //   shape: {
  //     kind: "disk",
  //     center: [268, 554, 280],
  //     radius: 100,
  //     normal: [0, 1, 0],
  //   },
  //   material: light,
  // });

  world.push({
    kind: "geom",
    // flip_face: true,
    transforms: [
      {
        kind: "rotate",
        angle: 90,
        axis: [1, 0, 0],
      },
      {kind: "translate", offset: [268, 554, 280]},
    ],
    shape: {
      // kind: "disk",
      // center: [268, 554, 280],
      // radius: 100,
      // normal: [0, 1, 0],
      // kind: "rect",
      // v0: [213, 554, 227],
      // v1: [343, 554, 332],
      kind: "regular_polygon",
      // center: [268, 554, 280],
      center: [0, 0, 0],
      radius: 100,
      num_sides: 6,
    },
    material: light,
    area_light: {},
  });

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, 555, 0],
      v1: [555, 555, 555],
    },
    material: white,
  });

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, 0, 0],
      v1: [555, 0, 555],
    },
    material: white,
  });

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, 0, 555],
      v1: [555, 555, 555],
    },
    material: white,
  });

  // cubes
  let cube1 = make_primitive({
    kind: "geom",
    transforms: [
      {
        kind: "translate",
        offset: [130, 0, 65],
      },
      {
        kind: "rotate",
        axis: [0, 1, 0],
        angle: -18.0,
      },
    ],
    shape: {
      kind: "cube",
      p_min: [0, 0, 0],
      p_max: [165, 165, 165],
    },
    material: white,
  });

  let cube2 = make_primitive({
    kind: "geom",
    transforms: [
      {
        kind: "translate",
        offset: [265, 0, 295],
      },
      {
        kind: "rotate",
        axis: [0, 1, 0],
        angle: 15.0,
      },
    ],
    shape: {
      kind: "cube",
      p_min: [0, 0, 0],
      p_max: [165, 330, 165],
    },
    material: white,
  });

  world.push(cube1, cube2);

  return world;
}

export default make_project({
  name: "cornell_box",
  settings: {
    output_dir: "./output/mis",
    height: 500,
    width: 500,
    nsamples: 500,
    // nsamples: 10,
    // nsamples: 3,
    max_depth: 10,
    mis_weight: 0.5,
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
      // lights,
      environments: [
        // {
        // // l: [0.7, 0.8, 1.0],
        // l: [0.0, 0.0, 0.0],
        // },
      ],
      world: [...create_world()],
    },
  ],
});
