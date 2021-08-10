use crate::lighting;
use crate::scene;
use crate::vector::Vec3;

/*
Warning: This is *slow*
only do it when we hit something
*/
#[allow(dead_code)]
fn calc_normal(sdf: &dyn Fn(Vec3) -> f32, p: Vec3) -> Vec3 {
    let eps = 0.001;
    let eps_x = Vec3::new(eps, 0.0, 0.0);
    let eps_y = Vec3::new(0.0, eps, 0.0);
    let eps_z = Vec3::new(0.0, 0.0, eps);

    let normal = Vec3::new(
        sdf(p + eps_x) - sdf(p - eps_x),
        sdf(p + eps_y) - sdf(p - eps_y),
        sdf(p + eps_z) - sdf(p - eps_z),
    );

    normal.normalize()
}

/*
 One sided version is faster,
 twice as much error
*/
fn calc_normal_eff(sdf: &dyn Fn(Vec3) -> f32, p: Vec3) -> Vec3 {
    let eps = 0.001;
    let eps_x = Vec3::new(eps, 0.0, 0.0);
    let eps_y = Vec3::new(0.0, eps, 0.0);
    let eps_z = Vec3::new(0.0, 0.0, eps);

    let sdf_p = sdf(p);

    let normal = Vec3::new(
        sdf_p - sdf(p - eps_x),
        sdf_p - sdf(p - eps_y),
        sdf_p - sdf(p - eps_z),
    );

    normal.normalize()
}

pub fn march(scene: &scene::Scene, view_dir: &Vec3) -> (f32, f32, f32) {
    let mut dist = 0.0;

    let mut steps = scene.max_steps;
    let mut radius = 0.0;
    for i in 1..scene.max_steps {
        dist += radius;
        radius = (scene.sdf)(scene.start + dist * view_dir);
        if dist > scene.max_dist || radius < scene.epsilon {
            steps = i;
            break;
        }
    }

    let end_pos = scene.start + dist * view_dir;

    let extra = match scene.extra_sdf {
        Some(sdf) => sdf(end_pos),
        None => 1.0,
    };

    let normal = calc_normal_eff(scene.sdf, end_pos);

    let light = (normal.x + normal.y + normal.z).abs() / 3.0;

    (scene.lighting_fn)(lighting::LightingInfo {
        end_pos,
        normal,
        ray_dist: dist / scene.max_dist,
        obj_dist: radius,
        light,
        extra,
        steps: (steps as f32) / (scene.max_steps as f32),
    })
}
