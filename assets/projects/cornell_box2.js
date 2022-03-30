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

  world.push(
    make_geometry({
      kind: "tags",
      tags: ["lights"],
      child: {
        kind: "flip_face",
        child: {
          kind: "disk",
          center: [268, 554, 280],
          radius: 100,
          normal: [0, 1, 0],
          // kind: "rect",
          // v0: [213, 554, 227],
          // v1: [343, 554, 332],
          // radius: 100,
          // material: light,
          material: light,
        },
      },
    })
  );

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

  // cube1
  world.push(
    make_geometry({
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
    })
  );

  // cylinder2
  world.push(
    make_geometry({
      kind: "transforms",
      params: [
        { kind: "translate", offset: [365, -40, 295] },
        { kind: "rotate", angle: -20, axis: "X" },
        { kind: "rotate", angle: 10, axis: "Z" },
      ],
      child: {
        kind: "cylinder",
        center0: [0, 0, 0],
        center1: [0, 330, 0],
        radius: 50,
        material: white,
      },
    })
  );

  return world;
}

export default make_project({
  name: "cornell_box2",
  settings: {
    output_dir: "./output",
    height: 500,
    width: 500,
    nsamples: 100,
    // nsamples: 10,
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
    // lights,
    sky: {
      kind: "solid",
      // background: [0.7, 0.8, 1.0],
      background: [0.0, 0.0, 0.0],
    },
    world: {
      // kind: "list",
      kind: "bvh",
      objects: create_world(),
      time0: 0,
      time1: 1.0,
    },
  },
});
