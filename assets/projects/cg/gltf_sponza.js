export default make_project({
  name: "gltf_sponza",
  settings: {
    output_dir: "./output/cg",
    width: 800,
    height: 800,
    nsamples: 10,
    max_depth: 40,
    mis_weight: 0.5,
  },
  scenes: [
    {
      transforms: [
        {
          kind: "rotate",
          axis: [0, 1, 0],
          angle: 90,
        },
      ],
      kind: "uri",
      uri: "assets:///models/no-sync/sponza.gltf",
    },
    {
      kind: "custom",
      camera: {
        look_from: [0, 2, 2],
        look_at: [0, 2, 0],
        view_up: [0, 1, 0],
        vertical_fov: 120,
      },
      environments: [
        {
          l: Vec3.mul([0.1, 0.1, 0.1], 10),
        },
      ],
    },
  ],
});
