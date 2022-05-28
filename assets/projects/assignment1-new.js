// x
const box_width = 556;
// y
const box_height = 556;
// z
const box_depth = 556;

export function create_world() {
  const red = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0.65, 0.05, 0.05],
    },
  });

  const white = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0.73, 0.73, 0.73],
    },
  });

  const green = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0.12, 0.45, 0.15],
    },
  });

  const strong_light = make_material({
    kind: "diffuse_light",
    emit: {
      kind: "constant_texture",
      color: [7.0, 7.0, 7.0],
    },
  });

  const world = make_geometry_list([]);

  // left
  world.push({
    kind: "rect",
    v0: [0, 0, 0],
    v1: [0, box_height, box_depth],
    material: red,
  });

  // right
  world.push({
    kind: "rect",
    v0: [box_width, 0, 0],
    v1: [box_width, box_height, box_depth],
    material: green,
  });

  world.push(
    make_geometry({
      kind: "tags",
      tags: ["lights"],
      child: {
        kind: "flip_face",
        child: {
          kind: "disk",
          properties: {
            sampler: {
              kind: "random",
            },
          },
          center: [box_width / 2, box_height - 1, box_depth / 2],
          radius: 0.2 * box_width,
          normal: [0, 1, 0],
          material: strong_light,
        },
      },
    })
  );

  // top (ceil)
  world.push({
    kind: "rect",
    v0: [0, box_height, 0],
    v1: [box_width, box_height, box_depth],
    material: white,
  });

  // bottom (floor)
  world.push({
    kind: "rect",
    v0: [0, 0, 0],
    v1: [box_width, 0, box_depth],
    material: white,
  });

  // back
  world.push({
    kind: "rect",
    v0: [0, 0, 0],
    v1: [box_width, box_height, 0],
    material: white,
  });

  // cube0
  world.push(
    make_geometry({
      kind: "transforms",
      params: [{kind: "translate", offset: [50, 0, 50]}],
      child: {
        kind: "cube",
        p_min: [0, 0, 0],
        p_max: [165, 165, 165],
        material: white,
      },
    })
  );

  // const cow_size = 60;
  // world.push({
  //   kind: "transforms",
  //   params: [
  //     {kind: "rotate", axis: "Y", angle: -180},
  //     {kind: "translate", offset: [1 * cow_size, 1 * cow_size, 2 * cow_size]},
  //   ],
  //   child: {
  //     kind: "mesh",
  //     from_obj: {
  //       file_path: "assets/models/cmu_cow/spot.obj",
  //     },
  //     material: {
  //       kind: "diffuse_light",
  //       emit: {
  //         kind: "image_texture",
  //         file_path: "assets/models/cmu_cow/spot_texture.png",
  //       },
  //     },
  //     load_options: {
  //       scale: cow_size,
  //     },
  //   },
  // });

  return world;
}

export default make_project({
  name: "assignment1-new",
  settings: {
    output_dir: "./output",
    height: 800,
    width: 800,
    nsamples: 5,
    max_depth: 15,
  },
  scene: {
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
    sky: {
      kind: "solid",
      background: [0.0, 0.0, 0.0],
    },
    world: {
      kind: "bvh",
      children: create_world(),
      time0: 0,
      time1: 1.0,
    },
  },
});
