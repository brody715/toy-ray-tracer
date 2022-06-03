export default make_project({
  name: "simple_cornell",
  settings: {
    output_dir: "./output/gltf",
    width: 800,
    height: 800,
    nsamples: 20,
    max_depth: 20,
    mis_weight: 1.0,
  },
  scenes: [
    {kind: "uri", uri: "assets:///models/cornell_box/simple.gltf"},
    {
      kind: "custom",
      camera: {
        look_from: [0, 0, 1],
        look_at: [0, 0, 0],
        view_up: [0, 1, 0],
        vertical_fov: 90,
      },
      environments: [{l: [0.1, 0.1, 0.1]}],
    },
  ],
});
