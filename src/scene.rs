use crate::raymarch::*;
use crate::vector::Vec3;

pub fn mandlebulb_scene_sdf(p: Vec3) -> f32 {
    mandlebulb_sdf(p, 100, 10.0, 4.0)
}

pub fn torus_scene_sdf(p: Vec3) -> f32 {
    torus_sdf(p - Vec3::new(0.0, 2.5, 0.0), 1.5, 0.4)
        .min(plane_sdf(p))
        .min(sphere_sdf(p))
}

pub fn gyroid_scene_sdf(p: Vec3) -> f32 {
    gyroid_sdf(p, 5.0, 1.5).max(plane_sdf_2(p, Vec3::new(0.5, 0.5, -0.5), -4.0))
}

pub fn example_scene_sdf(p: Vec3) -> f32 {
    sphere_sdf(p).min(plane_sdf(p))
}
