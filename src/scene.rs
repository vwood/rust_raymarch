use crate::raymarch::*;
use crate::vector::Vec3;

use serde::Deserialize;

use crate::lighting;
use crate::vector;

#[derive(Deserialize)]
#[serde(default)]
pub struct SceneDescription {
    pub sdf: String,
    pub width: u32,
    pub height: u32,
    pub camera_pos: (f32, f32, f32),
    pub look_at: (f32, f32, f32),
    pub lighting: String,
}

impl Default for SceneDescription {
    fn default() -> SceneDescription {
        SceneDescription {
            sdf: "default".to_string(),
            width: 640,
            height: 480,
            camera_pos: (0.5, 0.5, -2.0),
            look_at: (0.0, 0.0, 0.0),
            lighting: "default".to_string(),
        }
    }
}

/// Information needed to ray march
pub struct Scene {
    pub sdf: &'static (dyn Fn(Vec3) -> f32 + Sync),
    pub lighting_fn:
        &'static (dyn Fn(Vec3, Vec3, f32, f32, f32, f32, f32) -> (f32, f32, f32) + Sync),
    pub width: u32,
    pub height: u32,
    pub start: Vec3,
    pub max_steps: u32,
    pub max_dist: f32,
    pub epsilon: f32,
    pub extra_sdf: Option<&'static (dyn Fn(Vec3) -> f32 + Sync)>,
}

impl From<SceneDescription> for Scene {
    fn from(description: SceneDescription) -> Self {
        Scene {
            sdf: match description.sdf.as_str() {
                "torus" => &torus_scene_sdf,
                "mandlebulb" => &mandlebulb_scene_sdf,
                "gyroid" => &gyroid_scene_sdf,
                "example" => &example_scene_sdf,
                _ => &example_scene_sdf,
            },
            lighting_fn: match description.lighting.as_str() {
                "lighting2" => &lighting::simple_lighting_2,
                _ => &lighting::simple_lighting,
            },
            width: description.width,
            height: description.height,
            start: vector::Vec3::from(description.camera_pos),
            max_steps: 100,
            max_dist: 255.0,
            epsilon: 0.001,
            extra_sdf: Some(&mandlebulb_scene_sdf_iter),
        }
    }
}

pub fn mandlebulb_scene_sdf(p: Vec3) -> f32 {
    mandlebulb_sdf(p, 100, 10.0, 4.0)
}

pub fn mandlebulb_scene_sdf_iter(p: Vec3) -> f32 {
    mandlebulb_sdf_iter(p, 100, 10.0, 4.0)
}

pub fn torus_scene_sdf(p: Vec3) -> f32 {
    torus_sdf(p - Vec3::new(0.0, 2.5, 0.0), 1.5, 0.4)
        .min(plane_sdf(p, Vec3::new(0.0, 1.0, 0.0), 0.0))
        .min(sphere_sdf(p))
}

pub fn gyroid_scene_sdf(p: Vec3) -> f32 {
    gyroid_sdf(p, 5.0, 1.5).max(plane_sdf(p, Vec3::new(0.5, 0.5, -0.5), -4.0))
}

pub fn example_scene_sdf(p: Vec3) -> f32 {
    sphere_sdf(p).min(plane_sdf(p, Vec3::new(0.0, 1.0, 0.0), 0.0))
}
