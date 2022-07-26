export default make_project({
  name: "gltf_texture",
  settings: {
    output_dir: "./output/cg",
    width: 800,
    height: 800,
    nsamples: 500,
    max_depth: 20,
    mis_weight: 0.5,
  },
  scenes: [
    {
      kind: "uri",
      transforms: [{kind: "translate", offset: [-0.5, -0.5, 0]}],
      uri: "assets:///models/gltf-tutorials/simple_texture.gltf",
    },
    {
      kind: "custom",
      camera: {
        look_from: [0, 0, 0.8],
        look_at: [0, 0, 0],
        view_up: [0, 0.8, 0],
        vertical_fov: 90,
      },
      environments: [{l: [0.8, 0.8, 0.8]}],
    },
  ],
});
