const scenes = [].map((v) => make_scene(v));

export default make_project({
  name: "simple",
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
      vertical_fov: 20,
      aspect: 1,
      aperture: 0.0,
      focus_dist: 10.0,
      time0: 0.0,
      time1: 1.0,
    },
    environments: [
      {
        l: [0.1, 0.1, 0.1],
      },
    ],
    world: [
      {
        kind: "geom",
        shape: {
          kind: "sphere",
          center: [0.0, 2, 0],
          radius: 0.5,
        },
        material: {
          kind: "diffuse_light",
          emit: [1.0, 1.0, 1.0],
        },
        area_light: {},
      },
      {
        kind: "geom",
        shape: {
          kind: "sphere",
          center: [0.2, 0.2, 0.0],
          radius: 0.5,
        },
        material: {
          kind: "lambertian",
          albedo: {
            kind: "image_texture",
            uri: "assets:///textures/earthmap.jpg",
          },
        },
      },
      {
        kind: "geom",
        shape: {
          kind: "sphere",
          center: [0, -1, 0],
          radius: 0.5,
        },
        material: {
          kind: "lambertian",
          albedo: {
            kind: "constant_texture",
            value: [1.0, 0, 0],
          },
        },
      },
      {
        kind: "geom",
        shape: {
          kind: "sphere",
          center: [0, -1000, 0],
          radius: 1000,
        },
        material: {
          kind: "lambertian",
          albedo: [0.2, 0.3, 0.1],
        },
      },
    ],
  },
});
