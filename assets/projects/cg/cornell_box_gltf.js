export default make_project({
  name: "cornell_box_gltf",
  settings: {
    output_dir: "./output/cg",
    width: 800,
    height: 800,
    nsamples: 200,
    max_depth: 40,
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
        vertical_fov: 80,
      },
    },
  ],
});
