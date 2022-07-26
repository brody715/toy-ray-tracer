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
      value: [7.0, 7.0, 7.0],
    },
  });

  const world = make_primitive_list([]);

  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [555, 0, 0],
      v1: [555, 555, 555],
      material: green,
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

  world.push({
    kind: "geom",
    shape: {
      kind: "disk",
      center: [268, 554, 280],
      radius: 100,
      normal: [0, 1, 0],
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
      {kind: "translate", offset: [130, 0, 65]},
      {
        kind: "rotate",
        axis: [0, 1, 0],
        angle: -18,
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
      {kind: "rotate", axis: [0, 1, 0], angle: 15},
      {
        kind: "translate",
        offset: [285, 0, 295],
        // offset: [0, 0, 0],
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
    output_dir: "./output/cg",
    height: 500,
    width: 500,
    nsamples: 500,
    // nsamples: 10,
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
