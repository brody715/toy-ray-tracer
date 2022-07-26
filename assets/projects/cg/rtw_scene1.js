/// <reference path="../../schemas/global.d.ts" />

/** @typedef {import("../../schemas/project").JVec3F} JVec3F */

const scenes = [].map((v) => make_scene(v));

function random_float(min = 0, max = 1) {
  return min + Math.random() * (max - min);
}

export function create_world() {
  const origin = make_vec3f([4, 0.2, 0]);

  const world = make_primitive_list([]);

  world.push(
    make_primitive({
      kind: "geom",
      shape: {
        kind: "sphere",
        center: [0, -1000, 0],
        radius: 1000.0,
      },
      material: {
        kind: "lambertian",
        albedo: {
          kind: "checker_texture",
          even: {
            kind: "constant_texture",
            value: [0.2, 0.3, 0.1],
          },
          odd: {
            kind: "constant_texture",
            value: [0.9, 0.9, 0.9],
          },
        },
      },
    })
  );

  const n = 10;

  for (let a = -n; a < n; a++) {
    for (let b = -n; b < n; b++) {
      const choose_material = random_float();
      const center = make_vec3f([
        a + 0.9 * random_float(),
        0.2,
        b + 0.9 * random_float(),
      ]);

      if (Vec3.normalize(Vec3.sub(center, origin)) > 0.9) {
        if (choose_material < 0.8) {
          world.push(
            make_primitive({
              kind: "geom",
              shape: {
                kind: "sphere",
                center,
                radius: 0.2,
              },
              material: {
                kind: "lambertian",
                albedo: {
                  kind: "constant_texture",
                  value: [
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
            make_primitive({
              kind: "geom",
              shape: {
                kind: "sphere",
                center,
                radius: 0.2,
              },
              material: {
                kind: "metal",
                albedo: Vec3.random_vec3(0.5, 1.5),
                fuzz: 0.5 * random_float(),
              },
            })
          );
        } else {
          world.push(
            make_primitive({
              kind: "geom",
              shape: {
                kind: "sphere",
                center,
                radius: 0.2,
              },
              material: {
                kind: "dielectric",
                ir: 1.5,
                // kind: "transparent",
                // albedo: [1.0, 1.0, 1.0],
                // eta: 1.5,
                // roughness: 0.0,
              },
            })
          );
        }
      }
    }
  }

  world.push(
    make_primitive({
      kind: "geom",
      shape: {
        kind: "sphere",
        center: [0.0, 1.0, 0.0],
        radius: 1.0,
      },
      // material: {
      //   kind: "lambertian",
      //   albedo: [0.4, 0.0, 0.0],
      // },
      // material: {
      //   kind: "transparent",
      //   albedo: [1.0, 1.0, 1.0],
      //   eta: 1.5,
      //   roughness: 0.0,
      // },
      material: {
        kind: "dielectric",
        ir: 1.5,
      },
    })
  );

  world.push(
    make_primitive({
      kind: "geom",
      shape: {
        kind: "sphere",
        center: [-4, 1, 0],
        radius: 1,
      },
      material: {
        kind: "lambertian",
        albedo: {
          kind: "constant_texture",
          value: [0.4, 0.2, 0.1],
        },
      },
    })
  );

  world.push(
    make_primitive({
      kind: "geom",
      shape: {
        kind: "sphere",
        center: [4, 1, 0],
        radius: 1,
      },
      material: {
        kind: "metal",
        albedo: {
          kind: "constant_texture",
          value: [0.7, 0.6, 0.5],
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
  name: "rtw_scene1",
  settings: {
    output_dir: "./output/cg",
    width: size.width,
    height: size.height,
    nsamples: 200,
    max_depth: 40,
    mis_weight: 1.0,
  },
  scenes: [
    {
      kind: "custom",
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
      environments: [
        {
          l: [0.7, 0.8, 1.0],
        },
      ],
      world: create_world(),
    },
  ],
});
