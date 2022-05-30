/** @type {[number, number]} */
export const default_block_size = [10, 10];

function create_world(sampler) {
  const world = make_geometry_list([]);

  const white = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0.73, 0.73, 0.73],
    },
  });
  const blue = make_material({
    kind: "lambertian",
    albedo: {
      kind: "constant_texture",
      color: [0.0, 0.2, 0.7],
    },
  });

  const light_color = 300.0;
  const light = make_material({
    kind: "diffuse_light",
    emit: {
      kind: "constant_texture",
      color: [light_color, light_color, light_color],
    },
  });

  world.push({
    kind: "rect",
    v0: [-5555, 0, -5555],
    v1: [5555, 0, 5555],
    material: white,
  });

  let cube2 = make_primitive({
    kind: "translate",
    offset: [265, 0, 295],
    child: {
      kind: "rotate",
      axis: "Y",
      angle: 15.0,
      child: {
        kind: "cube",
        p_min: [0, 0, 0],
        p_max: [165, 330, 165],
        material: white,
      },
    },
  });

  world.push(cube2);

  world.push(
    make_primitive({
      kind: "tags",
      tags: ["lights"],
      child: {
        kind: "flip_face",
        child: {
          kind: "disk",
          properties: {
            sampler: sampler,
          },
          center: [600, 554 * 1.5, 268 * 5],
          radius: 100,
          normal: [0, 0, 1],
          material: light,
        },
      },
    })
  );

  return world;
}

export function make_assignment2({name, nsamples = 500, sampler}) {
  const world = create_world(sampler);
  return make_project({
    name,
    settings: {
      output_dir: "./output",
      height: 500,
      width: 500,
      nsamples: nsamples,
      max_depth: 15,
      weight: 0.5,
    },
    scene: {
      camera: {
        look_from: [278, 278 * 2, -1400],
        look_at: [278, 278, 0],
        view_up: [0, 1, 0],
        vertical_fov: 40.0,
        aspect: 1.0,
        aperture: 0.0,
        focus_dist: 10.0,
        time0: 0.0,
        time1: 0.0,
      },
      sky: {
        kind: "solid",
        background: [0.0, 0.0, 0.0],
      },
      world: {
        kind: "bvh",
        children: world,
        time0: 0,
        time1: 1.0,
      },
    },
  });
}
