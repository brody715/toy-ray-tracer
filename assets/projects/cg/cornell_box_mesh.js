// x
const box_width = 556;
// y
const box_height = 556;
// z
const box_depth = 556;

export function create_world() {
  const red = make_material({
    kind: "lambertian",
    albedo: [0.65, 0.05, 0.05],
  });

  const white = make_material({
    kind: "lambertian",
    albedo: [0.73, 0.73, 0.73],
  });

  const green = make_material({
    kind: "lambertian",
    albedo: [0.12, 0.45, 0.15],
  });

  const strong_light = make_material({
    kind: "diffuse_light",
    emit: Vec3.mul([1.0, 1.0, 1.0], 7),
  });

  const world = make_primitive_list([]);

  // left
  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, 0, 0],
      v1: [0, box_height, box_depth],
    },
    material: red,
  });

  // right
  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [box_width, 0, 0],
      v1: [box_width, box_height, box_depth],
    },
    material: green,
  });

  world.push(
    make_primitive({
      kind: "geom",
      transforms: [],
      // flip_face: true,
      shape: {
        kind: "disk",
        center: [box_width / 2, box_height - 1, box_depth / 2],
        radius: 0.2 * box_width,
        normal: [0, 1, 0],
      },
      // shape: {
      //   kind: "sphere",
      //   center: [
      //     box_width / 2,
      //     box_height - 1 + 0.1 * box_width,
      //     box_depth / 2,
      //   ],
      //   radius: 0.2 * box_width,
      // },
      area_light: {},
      material: strong_light,
    })
  );
  world.pop();

  world.push({
    kind: "geom",
    transforms: [
      {
        kind: "rotate",
        axis: [1, 0, 0],
        angle: 90,
      },
      {
        kind: "translate",
        offset: [box_width / 2, box_height - 1, box_depth / 2],
      },
    ],
    // flip_face: true,
    shape: {
      kind: "regular_polygon",
      radius: 0.2 * box_width,
      num_sides: 6,
    },
    area_light: {},
    material: strong_light,
  });
  // world.pop()

  // top (ceil)
  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, box_height, 0],
      v1: [box_width, box_height, box_depth],
    },
    material: white,
  });

  // bottom (floor)
  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, 0, 0],
      v1: [box_width, 0, box_depth],
    },
    material: white,
  });

  // back
  world.push({
    kind: "geom",
    shape: {
      kind: "rect",
      v0: [0, 0, 0],
      v1: [box_width, box_height, 0],
    },
    material: white,
  });

  // cube1
  world.push(
    make_primitive({
      kind: "container",
      transforms: [
        {kind: "rotate", axis: [0, 1, 0], angle: -18},
        {kind: "translate", offset: [0.6 * box_width, 0, 0.5 * box_depth]},
      ],
      children: [
        {
          kind: "geom",
          shape: {
            kind: "cube",
            p_min: [0, 0, 0],
            p_max: [165, 165, 165],
          },
          material: white,
        },
        {
          kind: "geom",
          transforms: [{kind: "translate", offset: [60, 250, 60]}],
          shape: {
            kind: "sphere",
            center: [0, 0, 0],
            radius: 50,
          },
          material: {
            // kind: "dielectric",
            // ir: 1.5,
            // kind: "transparent",
            kind: "metal",
            fuzz: 0.0,
            // fuzz: 0.0,
            // roughness: 0,
            // eta: 1.5,
            albedo: {
              kind: "constant_texture",
              value: Color.rgb2vec3(255, 0, 0),
            },
          },
        },
        {
          kind: "geom",
          transforms: [
            {kind: "rotate", axis: [0, 1, 0], angle: 0},
            {
              kind: "translate",
              offset: [60, 320, 60],
            },
          ],
          shape: {
            kind: "pyramid",
            v0: [0, 50, 0],
            v1: [-50, 0, 0],
            v2: [0, 0, 50],
            v3: [50, 0, 0],
          },
          material: {
            kind: "lambertian",
            albedo: Color.hex2vec3(0xed556a),
          },
        },
      ],
    })
  );

  {
    const height = 250;

    const m_cylinder = make_material({
      kind: "lambertian",
      albedo: {
        kind: "constant_texture",
        value: Color.hex2vec3(0x57c3c2),
      },
    });
    world.push({
      kind: "container",
      transforms: [
        {kind: "rotate", axis: [0, 1, 0], angle: 45},
        {
          kind: "translate",
          offset: [0.35 * box_width, 0.25 * box_height, 0.5 * box_depth],
        },
      ],
      children: [
        {
          kind: "geom",
          shape: {
            kind: "cylinder",
            center0: [0, 0, 0],
            center1: [0, height, 0],
            radius: 50,
          },
          material: m_cylinder,
        },
        {
          kind: "geom",
          shape: {
            kind: "cylinder",
            center0: [-height / 2, height / 2, 0],
            center1: [height / 2, height / 2, 0],
            radius: 50,
          },
          material: m_cylinder,
        },
        {
          kind: "geom",
          shape: {
            kind: "cylinder",
            center0: [0, height / 2, -height / 2],
            center1: [0, height / 2, height / 2],
            radius: 50,
          },
          material: m_cylinder,
        },
      ],
    });
  }

  const cow_size = 60;
  world.push({
    kind: "geom",
    transforms: [
      {kind: "scale", scale: [cow_size, cow_size, cow_size]},
      {kind: "rotate", axis: [0, 1, 0], angle: -180},
      {kind: "translate", offset: [1 * cow_size, 1 * cow_size, 2 * cow_size]},
    ],
    shape: {
      kind: "uri",
      uri: "assets:///models/cmu_cow/spot.obj",
    },
    material: {
      kind: "diffuse_light",
      emit: {
        kind: "image_texture",
        uri: "assets:///models/cmu_cow/spot_texture.png",
      },
    },
  });
  // world.pop();

  return world;
}

export default make_project({
  name: "cornell_box_mesh",
  settings: {
    output_dir: "./output/cg",
    height: 800,
    width: 800,
    nsamples: 500,
    max_depth: 50,
    mis_weight: 0.5,
  },
  scenes: [
    {
      kind: "custom",
      camera: {
        look_from: [box_width / 2, box_height / 2, 2.3 * box_depth],
        look_at: [box_width / 2, box_height / 2, 0],
        view_up: [0, 1, 0],
        vertical_fov: 40.0,
        aspect: 1.0,
        aperture: 0.0,
        focus_dist: 10.0,
        time0: 0.0,
        time1: 0.0,
      },
      // lights,
      environments: [
        // {
        //   l: [0.0, 0.0, 0.0],
        // },
      ],
      world: create_world(),
    },
  ],
});
