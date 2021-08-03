use crate::vector::Vec3;

#[allow(unused_variables)]
pub fn simple_lighting(
    end_pos: Vec3,
    normal: Vec3,
    ray_dist: f32,
    obj_dist: f32,
    light: f32,
    extra: f32,
    steps: f32,
) -> (f32, f32, f32) {
    (1.0 - (ray_dist).min(1.0), steps as f32, extra)
}

#[allow(unused_variables)]
pub fn simple_lighting_2(
    end_pos: Vec3,
    normal: Vec3,
    ray_dist: f32,
    obj_dist: f32,
    light: f32,
    extra: f32,
    steps: f32,
) -> (f32, f32, f32) {
    (
        1.0 - (ray_dist).min(1.0),
        steps as f32,
        (1.0 - (2.0 / -obj_dist).exp()) * light,
    )
}
