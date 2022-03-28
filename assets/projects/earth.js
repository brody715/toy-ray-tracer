export default make_project({
  name: "earth",
  settings: {
    output_dir: "./output",
    height: 800,
    width: 800,
    nsamples: 100,
    max_depth: 15,
  },
  scene: {
    camera: {
      look_from: [13, 2, 3],
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
    world: [
      {
        kind: "sphere",
        center: [0.0, 0.0, 0.0],
        radius: 2.0,
        material: {
          kind: "lambertian",
          albedo: {
            kind: "image_texture",
            file_path: "assets/textures/earthmap.jpg",
          },
        },
      },
    ],
  },
});
