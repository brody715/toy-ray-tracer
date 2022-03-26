import {
  CameraConfig,
  GeometryConfig,
  MaterialConfig,
  ProjectConfig,
  SceneConfig,
  Settings,
  SkyConfig,
  TextureConfig,
  Vec3F,
} from "./project";

declare global {
  function log(v: unknown): void;

  function make_project(project: ProjectConfig): string;

  function make_scene(scene: SceneConfig): SceneConfig;

  function make_geometry(geometry: GeometryConfig): GeometryConfig;

  function make_geometry_list(objects: GeometryConfig[]): GeometryConfig[];

  function make_material(material: MaterialConfig): MaterialConfig;

  function make_texture(texture: TextureConfig): TextureConfig;

  function make_camera(camera: CameraConfig): CameraConfig;

  function make_sky(sky: SkyConfig): SkyConfig;

  function make_settings(settings: Settings): Settings;

  function make_vec3f(v: Vec3F): Vec3F;
}
