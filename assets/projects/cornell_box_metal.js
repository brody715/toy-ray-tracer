export function create_world() {
  const m_red = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0.65, 0.05, 0.05],
    },
  });

  const m_white = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0.73, 0.73, 0.73],
    },
  });

  const m_green = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0.12, 0.45, 0.15],
    },
  });

  const m_light = make_material({
    kind: "diffuse_light",
    emit: {
      kind: "constant_texture",
      color: [7.0, 7.0, 7.0],
    },
  });

  const m_aluminu = make_material({
    kind: "metal",
    albedo: {
      kind: "constant_texture",
      color: [0.8, 0.85, 0.88],
    },
    fuzz: 0.0,
  });

  const m_glass = make_material({
    kind: "dielectric",
    ir: 1.5,
  });

  const world = make_geometry_list([]);

  world.push({
    kind: "rect",
    v0: [555, 0, 0],
    v1: [555, 555, 555],
    material: m_green,
  });

  world.push({
    kind: "rect",
    v0: [0, 0, 0],
    v1: [0, 555, 555],
    material: m_red,
  });

  const lights = make_geometry({
    kind: "list",
    children: [
      {
        kind: "flip_face",
        child: {
          kind: "rect",
          v0: [213, 554, 227],
          v1: [343, 554, 332],
          // radius: 100,
          material: m_light,
        },
      },
      {
        kind: "sphere",
        center: [190, 90, 190],
        radius: 90,
        material: m_light,
      },
    ],
  });

  world.push({
    kind: "flip_face",
    child: {
      kind: "rect",
      v0: [213, 554, 227],
      v1: [343, 554, 332],
      // radius: 100,
      material: m_light,
    },
  });

  world.push({
    kind: "rect",
    v0: [0, 555, 0],
    v1: [555, 555, 555],
    material: m_white,
  });

  world.push({
    kind: "rect",
    v0: [0, 0, 0],
    v1: [555, 0, 555],
    material: m_white,
  });

  world.push({
    kind: "rect",
    v0: [0, 0, 555],
    v1: [555, 555, 555],
    material: m_white,
  });

  // cubes
  let cube1 = make_geometry({
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
        material: m_aluminu,
      },
    },
  });

  let cube2 = make_geometry({
    kind: "sphere",
    center: [190, 90, 190],
    radius: 90,
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

  return { world, lights };
}

const { world, lights } = create_world();

export default make_project({
  name: "cornell_box_metal",
  settings: {
    output_dir: "./output",
    height: 500,
    width: 500,
    nsamples: 1000,
    // nsamples: 3,
    max_depth: 50,
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
    lights,
    sky: {
      kind: "solid",
      //   background: [0.7, 0.8, 1.0],
      background: [0.0, 0.0, 0.0],
    },
    world: {
      // kind: "list",
      kind: "bvh",
      children: world,
      time0: 0,
      time1: 1.0,
    },
  },
});
