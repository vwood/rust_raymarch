use crate::vector::Vec3;

#[allow(dead_code)]
pub fn sphere_sdf(p: Vec3) -> f32 {
    (p - Vec3::new(0.0, 0.0, 3.0)).length() - 2.0
}

#[allow(dead_code)]
pub fn torus_sdf(p: Vec3, dia: f32, thickness: f32) -> f32 {
    (((p.x.powf(2.0) + p.z.powf(2.0)).sqrt() - dia).powf(2.0) + p.y.powf(2.0)).sqrt() - thickness
}

#[allow(dead_code)]
pub fn plane_sdf(p: Vec3) -> f32 {
    p.y + 2.0
}

#[allow(dead_code)]
pub fn plane_sdf_2(p: Vec3, plane: Vec3, dist: f32) -> f32 {
    p.dot(&plane) - dist
}

#[allow(dead_code)]
pub fn mandlebulb_sdf(p: Vec3, iterations: u32, bailout: f32, power: f32) -> f32 {
    let mut z = p;
    let mut dr = 1.0;
    let mut r = 0.0;

    for _i in 1..iterations {
        r = z.length();
        if r > bailout {
            break;
        }

        // convert to polar coordinates
        let theta = (z.z / r).acos();
        let phi = z.y.atan2(z.x);

        dr = r.powf(power - 1.0) * power * dr + 1.0;

        // scale and rotate the point
        let zr = r.powf(power);
        let theta = theta * power;
        let phi = phi * power;

        // convert back to cartesian coordinates
        z =
            zr * Vec3::new(
                theta.sin() * phi.cos(),
                phi.sin() * theta.sin(),
                theta.cos(),
            ) + p;
    }

    0.5 * r.log(2.0) * r / dr
}

pub fn mandlebulb_sdf_itercount(p: Vec3, iterations: u32, bailout: f32, power: f32) -> f32 {
    let mut z = p;
    let mut dr = 1.0;
    let mut r;

    for i in 1..iterations {
        r = z.length();
        if r > bailout {
            return (i as f32) / (iterations as f32);
        }

        // convert to polar coordinates
        let theta = (z.z / r).acos();
        let phi = z.y.atan2(z.x);

        dr = r.powf(power - 1.0) * power * dr + 1.0;

        // scale and rotate the point
        let zr = r.powf(power);
        let theta = theta * power;
        let phi = phi * power;

        // convert back to cartesian coordinates
        z =
            zr * Vec3::new(
                theta.sin() * phi.cos(),
                phi.sin() * theta.sin(),
                theta.cos(),
            ) + p;
    }

    1.0
}

pub fn gyroid_sdf(p: Vec3, scale: f32, bias: f32) -> f32 {
    let p = p * scale;

    (Vec3::new(p.x.sin(), p.y.sin(), p.z.sin())
        .dot(&(Vec3::new(p.z.cos(), p.x.cos(), p.y.cos()) - bias))
        .abs()
        / scale
        - 0.2)
        * 0.8
}

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

pub fn march(
    scene_sdf: &dyn Fn(Vec3) -> f32,
    start: Vec3,
    view_dir: Vec3,
    max_steps: u32,
    max_dist: f32,
    epsilon: f32,
) -> (f32, f32, f32) {
    let mut dist = 0.0;

    let mut steps = max_steps;
    let mut radius = 0.0;
    for i in 1..max_steps {
        dist += radius;
        radius = scene_sdf(start + dist * view_dir);
        if dist > max_dist || radius < epsilon {
            steps = i;
            break;
        }
    }

    let iter = mandlebulb_sdf_itercount(start + dist * view_dir, 100, 10.0, 4.0);

    let normal = calc_normal_eff(scene_sdf, start + dist * view_dir);

    let _light = (normal.x + normal.y + normal.z).abs() / 3.0;

    (
        1.0 - (dist / max_dist).min(1.0),
        (steps as f32) / (max_steps as f32),
        iter,
        // (1.0 - (2.0 / -radius).exp()) * light,
    )
}
