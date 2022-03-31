function create_world() {
  const m_blue = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0, 0, 255],
    },
  });
  const world = make_geometry_list([]);
  world.push({
    kind: "transforms",
    params: [
      { kind: "rotate", angle: 60, axis: "Y" },
      { kind: "rotate", angle: -30, axis: "Z" },
    ],
    child: {
      kind: "triangle",
      v0: [0, 1, 0],
      v1: [-1, 0, 0],
      v2: [1, 0, 0],
      material: m_blue,
    },
  });
  // world.pop();

  world.push({
    kind: "transforms",
    params: [
      // { kind: "rotate", angle: 60, axis: "Y" },
      // { kind: "rotate", angle: -30, axis: "Z" },
      { kind: "rotate", angle: -20, axis: "X" },
    ],
    child: {
      kind: "cylinder",
      center0: [0, -1, 0],
      center1: [0, 1, 0],
      radius: 1,
      material: m_blue,
    },
  });
  world.pop();
  return world;
}

export default make_project({
  name: "triangles",
  settings: {
    output_dir: "./output",
    height: 400,
    width: 400,
    nsamples: 100,
    max_depth: 15,
  },
  scene: {
    camera: {
      look_from: [0, 0, 20],
      look_at: [0, 0, 0],
      view_up: [0, 1, 0],
      vertical_fov: 20.0,
      aspect: 1.0,
      aperture: 0.0,
      focus_dist: 10.0,
      time0: 0.0,
      time1: 0.0,
    },
    sky: {
      kind: "solid",
      background: [0.7, 0.8, 1.0],
    },
    world: {
      kind: "list",
      children: create_world(),
      time0: 0,
      time1: 1,
    },
  },
});
