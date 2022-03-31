const world = make_geometry({
  kind: "bvh",
  time0: 0,
  time1: 1,
  children: [
    {
      kind: "disk",
      center: [0, 0, 0],
      radius: 1,
      normal: [0, 0, 1],
      material: {
        kind: "lambertian",
        albedo: {
          kind: "constant_texture",
          color: [255, 0, 0],
        },
      },
    },
  ],
});

export default make_project({
  name: "disks",
  settings: {
    output_dir: "./output",
    height: 800,
    width: 800,
    nsamples: 1,
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
    world,
  },
});
