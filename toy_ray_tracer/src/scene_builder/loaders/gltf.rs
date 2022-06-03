use anyhow::{Context, Result};
use std::{path::Path, sync::Arc};

use crate::{
    core::{vec3, MaterialPtr, Point2f, SceneBundle, Spectrum, TexturePtr, Transform, Vec3f},
    materials::GltfPbrMaterial,
    primitives::GeometricPrimitive,
    shapes::{Triangle, TriangleMeshStorage},
    textures::{ConstantTexture, ImageTexture, ImageTextureParams},
};

pub fn load_gltf_scenes<P: AsRef<Path>>(path: P, transform: Transform) -> Result<Vec<SceneBundle>> {
    let g_scenes = easy_gltf::load(path).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut scene_bundles = Vec::new();

    for g_scene in g_scenes.iter() {
        let scene_bundle = load_scene(g_scene, transform.clone())?;
        scene_bundles.push(scene_bundle);
    }

    Ok(scene_bundles)
}

fn load_scene(g_scene: &easy_gltf::Scene, transform: Transform) -> Result<SceneBundle> {
    let mut bundle = SceneBundle::default();

    for g_model in g_scene.models.iter() {
        if easy_gltf::model::Mode::Triangles != g_model.mode() {
            log::warn!(
                "only triangles are supported, but found {:?}. Ignore ...",
                g_model.mode()
            );
            continue;
        }

        let vertex_indices = g_model.indices().context("no indices found")?;
        let vertices = g_model.vertices();

        let positions: Vec<Vec3f> = vertices
            .iter()
            .map(|v| {
                let p = v.position;
                Vec3f::new(p.x, p.y, p.z)
            })
            .collect();

        let normals: Vec<Vec3f> = if g_model.has_normals() {
            vertices
                .iter()
                .map(|v| {
                    let n = v.normal;
                    Vec3f::new(n.x, n.y, n.z)
                })
                .collect()
        } else {
            Vec::new()
        };

        let uvs = if g_model.has_tex_coords() {
            vertices
                .iter()
                .map(|v| {
                    let uv = v.tex_coords;
                    Point2f::new(uv.x, uv.y)
                })
                .collect()
        } else {
            Vec::new()
        };

        let mesh = Arc::new(TriangleMeshStorage::try_new(
            vertex_indices.len() / 3,
            vertex_indices.clone(),
            positions,
            normals,
            uvs,
        )?);

        let g_material = g_model.material();
        let material: MaterialPtr = {
            let pbr = &g_material.pbr;
            let emissive = &g_material.emissive;

            let roughness_factor = pbr.roughness_factor;
            let metallic_factor = pbr.metallic_factor;
            let emissive_factor =
                Spectrum::new(emissive.factor.x, emissive.factor.y, emissive.factor.z);

            let base_color_factor = Spectrum::new(
                pbr.base_color_factor.x,
                pbr.base_color_factor.y,
                pbr.base_color_factor.z,
            );

            let base_color: TexturePtr<Spectrum> = match &pbr.base_color_texture {
                Some(g_base_color_texture) => {
                    let image_texture = ImageTexture::<Spectrum>::from_rgba_image(
                        &g_base_color_texture,
                        ImageTextureParams {
                            scale: base_color_factor,
                            flip: false,
                        },
                    );

                    Arc::new(image_texture)
                }
                None => Arc::new(ConstantTexture::new(base_color_factor)),
            };

            let emissive_color: TexturePtr<Spectrum> = match &emissive.texture {
                Some(g_emissive_texture) => {
                    let image_texture = ImageTexture::<Spectrum>::from_rgb_image(
                        &g_emissive_texture,
                        ImageTextureParams {
                            scale: emissive_factor,
                            flip: false,
                        },
                    );

                    Arc::new(image_texture)
                }
                None => Arc::new(ConstantTexture::new(emissive_factor)),
            };

            let metallic: TexturePtr<f32> = match &pbr.metallic_texture {
                Some(g_metallic_texture) => {
                    let image_texture = ImageTexture::<f32>::from_gray_image(
                        &g_metallic_texture,
                        ImageTextureParams {
                            scale: vec3::scalar(metallic_factor),
                            flip: false,
                        },
                    );

                    Arc::new(image_texture)
                }
                None => Arc::new(ConstantTexture::new(metallic_factor)),
            };

            let roughness: TexturePtr<f32> = match &pbr.roughness_texture {
                Some(g_roughness_texture) => {
                    let image_texture = ImageTexture::<f32>::from_gray_image(
                        &g_roughness_texture,
                        ImageTextureParams {
                            scale: vec3::scalar(roughness_factor),
                            flip: false,
                        },
                    );

                    Arc::new(image_texture)
                }
                None => Arc::new(ConstantTexture::new(roughness_factor)),
            };

            let material = Arc::new(GltfPbrMaterial::new(
                1.5,
                base_color,
                metallic,
                roughness,
                emissive_color,
            ));

            if log::max_level() >= log::Level::Trace {
                log::trace!("base_color: {:?}", pbr.base_color_factor)
            }

            material
        };

        let prims = &mut bundle.primitives;
        for id in 0..mesh.n_triangles {
            let triangle = Arc::new(Triangle::new(id, mesh.clone()));

            let prim = Arc::new(GeometricPrimitive::new(
                triangle,
                transform.clone(),
                material.clone(),
            ));

            prims.push(prim);
        }
    }

    Ok(bundle)
}
