use crate::vector::Vec3;

pub struct LightingInfo {
    pub end_pos: Vec3,
    pub normal: Vec3,
    pub ray_dist: f32,
    pub obj_dist: f32,
    pub light: f32,
    pub extra: f32,
    pub steps: f32,
}

pub fn simple_lighting(info: LightingInfo) -> (f32, f32, f32) {
    (
        1.0 - (info.ray_dist).min(1.0),
        info.steps as f32,
        info.extra,
    )
}

pub fn simple_lighting_2(info: LightingInfo) -> (f32, f32, f32) {
    (
        1.0 - (info.ray_dist).min(1.0),
        info.steps as f32,
        (1.0 - (2.0 / -info.obj_dist).exp()) * info.light,
    )
}
