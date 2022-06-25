export const smooth_material = make_material({
  kind: "gltf_pbr",
  base_color: [1, 1, 1],
  metallic: 0.9,
  roughness: 0.01,
});

export const rough_material = make_material({
  kind: "gltf_pbr",
  base_color: [1, 1, 1],
  metallic: 1.0,
  roughness: 0.1,
});

function create_world({board_material}) {
  const world = make_primitive_list([]);

  const m_light = make_material({
    kind: "diffuse_light",
    emit: Vec3.mul([0.8, 0.6, 0.3], 1),
  });

  const s_6_polygon = make_shape({
    kind: "regular_polygon",
    radius: 10,
    num_sides: 6,
  });

  /** @type {import("../../schemas/project").TransformConfig[]} */
  const light_transform = [
    {kind: "rotate", axis: [1, 0, 0], angle: 60},
    {
      kind: "translate",
      offset: [0, -25, -20],
    },
  ];
  // lights
  {
    world.push({
      kind: "geom",
      transforms: [
        ...light_transform,
        {kind: "translate", offset: [-80, 70, 0]},
      ],
      shape: {
        kind: "regular_polygon",
        radius: 5,
        num_sides: 4,
      },
      material: {
        kind: "diffuse_light",
        emit: Vec3.mul(Color.hex2vec3(0xffa9d4), 1),
      },
      area_light: {},
    });
    world.push({
      kind: "geom",
      transforms: [
        ...light_transform,
        {kind: "translate", offset: [-40, 70, 0]},
      ],
      shape: {
        kind: "regular_polygon",
        radius: 10,
        num_sides: 5,
      },
      material: {
        kind: "diffuse_light",
        emit: Vec3.mul([0.8, 0.6, 0.3], 1),
      },
      area_light: {},
    });
    world.push({
      kind: "geom",
      transforms: [
        ...light_transform,
        {kind: "translate", offset: [10, 70, 0]},
      ],
      shape: {
        kind: "regular_polygon",
        radius: 15,
        num_sides: 6,
      },
      material: {
        kind: "diffuse_light",
        emit: Color.hex2vec3(0xc8ffaa),
      },
      area_light: {},
    });
    world.push({
      kind: "geom",
      transforms: [
        ...light_transform,
        {kind: "translate", offset: [70, 70, 0]},
      ],
      shape: {
        kind: "regular_polygon",
        radius: 20,
        num_sides: 8,
      },
      material: {
        kind: "diffuse_light",
        emit: Color.hex2vec3(0x7eadc1),
      },
      area_light: {},
    });
  }

  const s_board_z = -20;

  world.push({
    transforms: [
      {
        kind: "rotate",
        axis: [1, 0, 0],
        angle: 30,
      },
      {kind: "translate", offset: [0, 0, 0]},
    ],
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [-100, s_board_z, -40],
      v1: [100, s_board_z, 40],
    },
    material: board_material,
  });

  return world;
}

export function make_mis_scene({
  name,
  nsamples = 100,
  mis_weight,
  board_material,
}) {
  return make_project({
    name,
    settings: {
      output_dir: "./output/assignment3",
      height: 500,
      width: 500,
      nsamples,
      max_depth: 40,
      mis_weight,
    },
    scenes: [
      {
        kind: "custom",
        camera: {
          look_from: [0, 0, 100],
          look_at: [0, 0, 0],
          view_up: [0, 1, 0],
          aspect: 1.0,
          focus_dist: 10,
          aperture: 0.0,
          vertical_fov: 90,
        },
        world: create_world({board_material}),
        environments: [
          //   {
          //     l: Vec3.mul([0.8, 0.6, 0.3], 1),
          //   },
        ],
      },
    ],
  });
}
