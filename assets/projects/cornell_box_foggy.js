export function create_world() {
  const red = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0.65, 0.05, 0.05],
    },
  });

  const white = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0.73, 0.73, 0.73],
    },
  });

  const green = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0.12, 0.45, 0.15],
    },
  });

  const light = make_material({
    kind: "diffuse_light",
    emit: {
      kind: "constant_texture",
      color: [7.0, 7.0, 7.0],
    },
  });

  const world = make_geometry_list([]);

  world.push({
    kind: "rect",
    v0: [555, 0, 0],
    v1: [555, 555, 555],
    material: green,
  });

  world.push({
    kind: "rect",
    v0: [0, 0, 0],
    v1: [0, 555, 555],
    material: red,
  });

  world.push({
    kind: "flip_face",
    child: {
      kind: "rect",
      v0: [113, 554, 127],
      v1: [443, 554, 432],
      // radius: 100,
      material: light,
    },
  });

  world.push({
    kind: "rect",
    v0: [0, 555, 0],
    v1: [555, 555, 555],
    material: white,
  });

  world.push({
    kind: "rect",
    v0: [0, 0, 0],
    v1: [555, 0, 555],
    material: white,
  });

  world.push({
    kind: "rect",
    v0: [0, 0, 555],
    v1: [555, 555, 555],
    material: white,
  });

  // cubes
  let cube1 = make_geometry({
    kind: "translate",
    offset: [130, 0, 65],
    child: {
      kind: "rotate",
      axis: "Y",
      angle: -18.0,
      child: {
        kind: "cube",
        p_min: [0, 0, 0],
        p_max: [165, 165, 165],
        material: white,
      },
    },
  });

  let cube2 = make_geometry({
    kind: "translate",
    offset: [265, 0, 295],
    child: {
      kind: "rotate",
      axis: "Y",
      angle: 15.0,
      child: {
        kind: "cube",
        p_min: [0, 0, 0],
        p_max: [165, 330, 165],
        material: white,
      },
    },
  });

  cube1 = make_geometry({
    kind: "constant_medium",
    boundary: cube1,
    density: 0.01,
    texture: {
      kind: "constant_texture",
      color: [1.0, 1.0, 1.0],
    },
  });

  cube2 = make_geometry({
    kind: "constant_medium",
    boundary: cube2,
    density: 0.01,
    texture: {
      kind: "constant_texture",
      color: [0.0, 0.0, 0.0],
    },
  });

  world.push(cube1, cube2);

  return world;
}

export default make_project({
  name: "cornell_box_foggy",
  settings: {
    output_dir: "./output",
    height: 800,
    width: 800,
    nsamples: 100,
    max_depth: 15,
  },
  scene: {
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
    sky: {
      kind: "solid",
      //   background: [0.7, 0.8, 1.0],
      background: [0.0, 0.0, 0.0],
    },
    world: {
      // kind: "list",
      kind: "bvh",
      children: create_world(),
      time0: 0,
      time1: 1.0,
    },
  },
});
