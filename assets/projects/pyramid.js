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

  const pyramid_builtin = make_geometry({
    kind: "pyramid",
    v0: [0, 1, 0],
    v1: [-1, 0, 0],
    v2: [1, 0, 0],
    v3: [0, 0, -1],
    material: m_pink,
  });

  const pyramid_mesh_file = make_geometry({
    kind: "mesh",
    from_obj: {
      file_path: "assets/models/simple/pyramid.obj",
    },
    material: m_pink,
  });

  const pyramid_mesh_string = make_geometry({
    kind: "mesh",
    from_obj: {
      raw_string: `
      v 0.0 1.0 0.0
      v -1.0 0.0 0.0
      v 1.0 0.0 0.0
      v 0.0 0.0 -1.0
      f 1 2 3
      f 1 2 4
      f 1 3 4
      f 2 3 4
      `,
    },
    material: m_pink,
  });

  world.push(pyramid_mesh_string);
  return world;
}

const size = Utils.make_screen_size({
  width: 800,
  height: 800,
});

export default make_project({
  name: "pyramid",
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
