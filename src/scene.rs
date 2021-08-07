use crate::raymarch::*;
use crate::vector::Vec3;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Scene {
    pub sdf: String,
    pub width: u32,
    pub height: u32,
    pub camera_pos: (f32, f32, f32),
    pub look_at: (f32, f32, f32),
    pub lighting: String,
}

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            sdf: "default".to_string(),
            width: 640,
            height: 480,
            camera_pos: (0.0, 0.0, -5.0),
            look_at: (0.0, 0.0, 0.0),
            lighting: "default".to_string(),
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
