import {
  CameraConfig,
  PrimitiveConfig,
  MaterialConfig,
  ProjectConfig,
  SceneConfig,
  Settings,
  EnvironmentConfig,
  TextureConfig,
} from "./project";

type Vec3F = [number, number, number];
type Vec2F = [number, number];

declare global {
  function log(v: unknown): void;

  function make_project(project: ProjectConfig): string;

  function make_scene(scene: SceneConfig): SceneConfig;

  function make_primitive(primitive: PrimitiveConfig): PrimitiveConfig;

  function make_primitive_list(children: PrimitiveConfig[]): PrimitiveConfig[];

  function make_material(material: MaterialConfig): MaterialConfig;

  function make_texture(texture: TextureConfig): TextureConfig;

  function make_camera(camera: CameraConfig): CameraConfig;

  function make_settings(settings: Settings): Settings;

  function make_vec3f(v: Vec3F): Vec3F;

  class Color {
    static rgb2vec3(r: number, g: number, b: number): Vec3F;
    static hex2vec3(hex: number): Vec3F;
  }

  class Vec3 {
    static random_f32(min?: number, max?: number): number;
    static random_vec3(min?: number, max?: number): Vec3F;
    static sub(v1: Vec3F, v2: Vec3F): Vec3F;
    static add(v1: Vec3F, v2: Vec3F): Vec3F;
    static dot(v1: Vec3F, v2: Vec3F): Vec3F;
    static normalize(v: Vec3F): number;
    static mul(v1: Vec3F, v2: Vec3F | number): Vec3F;
  }

  class Utils {
    static make_screen_size(size: {width: number; height: number}): {
      width: number;
      height: number;
      aspect: () => number;
    };
  }
}
