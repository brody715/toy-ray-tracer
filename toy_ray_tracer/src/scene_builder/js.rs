use anyhow::{Context, Ok};
use rquickjs::{FromJs, Module, Runtime};

use super::ProjectConfig;

fn create_internal_module<'js>(
    ctx: rquickjs::Ctx<'js>,
    object_names: &[&str],
) -> rquickjs::Module<'js, rquickjs::Loaded<rquickjs::Script>> {
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

    let module = Module::new(ctx, "inner", script).unwrap();
    return module;
}

pub fn load_from_js(script: &str) -> anyhow::Result<ProjectConfig> {
    let object_names = vec![
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

    let rt = Runtime::new()?;
    let ctx = rquickjs::Context::full(&rt)?;

    let project_config = ctx.with(|ctx| -> anyhow::Result<ProjectConfig> {
        let inner_module = create_internal_module(ctx, &object_names);

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

        let m = rquickjs::Module::new(ctx, "script", script)?;

        let str_value: rquickjs::Value = m.eval()?.get("default").unwrap();
        let str: String = String::from_js(ctx, str_value)?;

        let project_config: ProjectConfig =
            serde_json::from_str(&str).context(format!("error in parsing json"))?;

        Ok(project_config)
    })?;

    Ok(project_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script() {
        let project_config = load_from_js(
            r#"
        const a = make_scene({});
        export default make_project({})
        "#,
        )
        .unwrap();

        println!("{:?}", project_config);
    }
}
