export default make_project({
  name: "simple_cornell",
  settings: {
    output_dir: "./output/gltf",
    width: 800,
    height: 800,
    nsamples: 20,
    max_depth: 20,
    mis_weight: 0.5,
  },
  scenes: [
    {kind: "uri", uri: "assets:///models/cornell_box/simple.gltf"},
    {
      kind: "custom",
      camera: {
        look_from: [0, 1, 2],
        look_at: [0, 1, 0],
        view_up: [0, 1, 0],
        vertical_fov: 90,
      },
      // environments: [{l: Vec3.mul([0.1, 0.1, 0.1], 1)}],
    },
  ],
});
