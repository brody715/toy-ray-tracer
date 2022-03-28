/// <reference path="../schemas/global.d.ts" />

/** @typedef {import("../schemas/project").Vec3F} Vec3F */

const scenes = [].map((v) => make_scene(v));

function random_float(min = 0, max = 1) {
  return min + Math.random() * (max - min);
}

/** @returns {Vec3F} */
function random_vec3(min = 0, max = 1) {
  return [
    random_float(min, max),
    random_float(min, max),
    random_float(min, max),
  ];
}

/** @returns {Vec3F} */
function vec3_sub(v1, v2) {
  return [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]];
}

/** @returns {Vec3F} */
function vec3_add(v1, v2) {
  return [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]];
}

function vec3_dot(v1, v2) {
  return v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
}

function vec3_normalize(v) {
  return Math.sqrt(vec3_dot(v, v));
}

export function create_world() {
  const origin = [4, 0.2, 0];

  const world = make_geometry_list([]);

  world.push(
    make_geometry({
      kind: "sphere",
      center: [0, -1000, 0],
      radius: 1000.0,
      material: {
        kind: "lambertian",
        albedo: {
          kind: "checker_texture",
          even: {
            kind: "constant_texture",
            color: [0.2, 0.3, 0.1],
          },
          odd: {
            kind: "constant_texture",
            color: [0.9, 0.9, 0.9],
          },
        },
      },
    })
  );

  for (let a = -10; a < 10; a++) {
    for (let b = -10; b < 10; b++) {
      const choose_material = random_float();
      const center = make_vec3f([
        a + 0.9 * random_float(),
        0.2,
        b + 0.9 * random_float(),
      ]);

      if (vec3_normalize(vec3_sub(center, origin)) > 0.9) {
        if (choose_material < 0.8) {
          world.push(
            make_geometry({
              kind: "moving_sphere",
              center0: center,
              center1: vec3_add(center, [0, random_float(0, 0.5), 0]),
              time0: 0.0,
              time1: 1.0,
              radius: 0.2,
              material: {
                kind: "lambertian",
                albedo: {
                  kind: "constant_texture",
                  color: [
                    random_float() * random_float(),
                    random_float() * random_float(),
                    random_float() * random_float(),
                  ],
                },
              },
            })
          );
        } else if (choose_material < 0.95) {
          world.push(
            make_geometry({
              kind: "sphere",
              center,
              radius: 0.2,
              material: {
                kind: "metal",
                albedo: {
                  kind: "constant_texture",
                  color: random_vec3(0.5, 1.5),
                },
                fuzz: 0.5 * random_float(),
              },
            })
          );
        } else {
          world.push(
            make_geometry({
              kind: "sphere",
              center,
              radius: 0.2,
              material: {
                kind: "dielectric",
                ir: 1.5,
              },
            })
          );
        }
      }
    }
  }

  world.push(
    make_geometry({
      kind: "sphere",
      center: [0.0, 1.0, 0.0],
      radius: 1.0,
      material: {
        kind: "dielectric",
        ir: 1.5,
      },
    })
  );

  world.push(
    make_geometry({
      kind: "sphere",
      center: [-4, 1, 0],
      radius: 1,
      material: {
        kind: "lambertian",
        albedo: {
          kind: "constant_texture",
          color: [0.4, 0.2, 0.1],
        },
      },
    })
  );

  world.push(
    make_geometry({
      kind: "sphere",
      center: [4, 1, 0],
      radius: 1,
      material: {
        kind: "metal",
        albedo: {
          kind: "constant_texture",
          color: [0.7, 0.6, 0.5],
        },
        fuzz: 0.0,
      },
    })
  );

  return world;
}

const size = {
  width: 1280,
  height: 720,
  aspect: function () {
    return this.width / this.height;
  },
};

export default make_project({
  name: "scene1",
  settings: {
    output_dir: "./output",
    width: size.width,
    height: size.height,
    nsamples: 20,
    max_depth: 15,
  },
  scene: {
    camera: {
      look_from: [13, 2, 3],
      look_at: [0, 0, 0],
      view_up: [0, 1, 0],
      vertical_fov: 20,
      aspect: size.aspect(),
      aperture: 0.0,
      focus_dist: 10.0,
      time0: 0.0,
      time1: 1.0,
    },
    sky: {
      kind: "solid",
      background: [0.7, 0.8, 1.0],
    },
    world: create_world(),
  },
});
