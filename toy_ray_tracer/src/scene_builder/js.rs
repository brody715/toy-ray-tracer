pub(super) fn create_internal_module_script() -> String {
    let object_names: Vec<&str> = vec![
        "scene",
        "geometry_list",
        "geometry",
        "material",
        "texture",
        "camera",
        "sky",
        "settings",
        "vec3f",
    ];
    let mut script = String::from(
        r#"
export function log(v) {
    log_string(JSON.stringify(v));
}
export function make_project(v) { return JSON.stringify(v); }
"#,
    );

    for name in object_names.iter() {
        script.push_str(&format!("export function make_{}(v) {{ return v; }}", name));
    }

    let helper = String::from(
        r#"
export class Color {
    static rgb2vec3(r, g, b) {
        return [r / 255, g / 255, b / 255];
    }

    static hex2vec3(hex) {
        const b = hex & 0xff;
        hex >>= 8;
        const g = hex & 0xff;
        hex >>= 8;
        const r = hex & 0xff;
        return Color.rgb2vec3(r, g, b);
    }
};

export class Vec3 {
    static random_f32(min = 0, max = 1) {
        return min + Math.random() * (max - min);
    }

    static random_vec3(min = 0, max = 1) {
        return [
            random_f32(min, max),
            random_f32(min, max),
            random_f32(min, max),
        ];
    }

    static sub(v1, v2) {
        return [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]];
    }

    static add(v1, v2) {
        return [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]];
    }

    static dot(v1, v2) {
        return v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
    }

    static normalize(v) {
        return Math.sqrt(Vec3.dot(v, v));
    }
};

export class Utils {
    static make_screen_size(size) {
        return {
            width: size.width,
            height: size.height,
            aspect: function () {
              return this.width / this.height;
            },
        }
    }
}
    "#,
    );

    script.push_str(&helper);

    return script;
}

#[cfg(not(feature = "quickjs"))]
pub mod nodejs {
    use std::{
        fs::File,
        io::{BufReader, Write},
        process::{Command, Stdio},
    };

    use anyhow::Context;

    use crate::scene_builder::ProjectConfig;

    use super::create_internal_module_script;

    use tempdir::TempDir;

    pub fn load_from_js<P: AsRef<Path>>(script: &str, _path: P) -> anyhow::Result<ProjectConfig> {
        let inner_module_script = create_internal_module_script();

        let mut module_script = String::from("");
        module_script.push_str(&inner_module_script);
        module_script.push('\n');
        module_script.push_str(script);

        let dir = TempDir::new("ray_tracing")?;
        println!("default dir: {}", dir.path().display());

        let module_file_path = dir.path().join("mod.mjs");
        let mut module_file = File::create(module_file_path)?;
        write!(module_file, "{}", module_script)?;
        module_file.sync_all()?;

        let outfile_path = dir.path().join("out.json");

        let wrapper_script = format!(
            r#"import config from './mod.mjs';
               import fs from 'fs';
               fs.writeFileSync('{}', config);
        "#,
            outfile_path.display(),
        );

        // sleep(Duration::from_secs(1000));

        // start nodejs to run
        let mut p = Command::new("node")
            .current_dir(dir.path())
            .arg("--input-type=module")
            // .arg(AsRef::<OsStr>::as_ref(&wrapper_file_path))
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?;

        write!(p.stdin.as_mut().context("no stdin")?, "{}", wrapper_script)?;

        let out_file = File::open(outfile_path)?;
        let reader = BufReader::new(out_file);

        let project_config: ProjectConfig =
            serde_json::from_reader(reader).context(format!("error in parsing json"))?;

        dir.close()?;

        Ok(project_config)
    }
}

#[cfg(feature = "quickjs")]
pub mod quickjs {
    use std::path::Path;

    use anyhow::{Context, Ok};
    use rquickjs::{FileResolver, FromJs, Module, Runtime, ScriptLoader};

    use super::create_internal_module_script;
    use crate::scene_builder::ProjectConfig;

    pub fn load_from_js<P: AsRef<Path>>(script: &str, path: P) -> anyhow::Result<ProjectConfig> {
        // let script_dir = path
        //     .as_ref()
        //     .parent()
        //     .context("failed to load parent of path")?;
        let rt = Runtime::new()?;
        let ctx = rquickjs::Context::full(&rt)?;

        {
            let resolver = (FileResolver::default().with_path("./"),);
            let loader = (ScriptLoader::default(),);
            rt.set_loader(resolver, loader);
        }

        let project_config = ctx.with(|ctx| -> anyhow::Result<ProjectConfig> {
            let inner_module_script = create_internal_module_script();
            let inner_module = Module::new(ctx, "inner", inner_module_script).unwrap();

            let inner_module = inner_module.eval()?;

            let global = ctx.globals();
            for ri in inner_module.entries() {
                let result = ri.unwrap();
                let name: String = result.0;
                let value: rquickjs::Value = result.1;
                global.set(name, value).unwrap();
            }

            global.set(
                "log_string",
                rquickjs::Func::new("log_string", move |v: String| println!("{:?}", v)),
            )?;

            let script_path = path.as_ref().to_string_lossy().to_string();

            let m = rquickjs::Module::new(ctx, script_path, script)?;

            let str_value: rquickjs::Value = m.eval()?.get("default").unwrap();
            let str: String = String::from_js(ctx, str_value)?;

            let project_config: ProjectConfig =
                serde_json::from_str(&str).context(format!("error in parsing json"))?;

            Ok(project_config)
        })?;

        Ok(project_config)
    }
}

#[cfg(feature = "quickjs")]
pub use quickjs::load_from_js;

#[cfg(not(feature = "quickjs"))]
pub use nodejs::load_from_js;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script() {
        let project_config = load_from_js(
            r#"
    const a = make_scene({});
    export default make_project({
        name: "test",
        settings: {
            output_dir: './output',
            height: 800,
            width: 800,
            nsamples: 15,
            max_depth: 15
        },
        scene: {
            camera: {
                look_from: [0, 0, 10],
                look_at: [0, 0, 0],
                view_up: [0, 1, 0],
                vertical_fov: 20.0,
                aspect: 1.0,
                aperture: 0.0,
                focus_dist: 10.0,
                time0: 0.0,
                time1: 0.0,
              },
              sky: {
                kind: "solid",
                background: [0.7, 0.8, 1.0],
              },
              world: []
        }
    });
    "#,
            ".",
        )
        .unwrap();

        assert_eq!(project_config.name, "test");
        assert_eq!(project_config.settings.height, 800);
        assert_eq!(project_config.scene.camera.focus_dist, 10.0);

        println!("{:?}", project_config);
    }
}
