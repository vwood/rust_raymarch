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
pub fn plane_sdf(p: Vec3, plane: Vec3, dist: f32) -> f32 {
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

#[allow(dead_code)]
pub fn mandlebulb_sdf_iter(p: Vec3, iterations: u32, bailout: f32, power: f32) -> f32 {
    let mut z = p;
    let mut dr = 1.0;
    let mut r;

    let mut steps = iterations;
    for i in 1..iterations {
        r = z.length();
        if r > bailout {
            steps = i;
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

    (steps as f32) / (iterations as f32)
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

/// Based on cantor pairing function expanded to a triple
#[allow(dead_code)]
fn cantor_hash(grid: Vec3) -> f32 {
    let (k1, k2, k3) = (grid.x, grid.y, grid.z);

    let pair = 0.5 * (k1 + k2) * (k1 + k2 + 1.0) + k2;

    0.5 * (pair + k3) * (pair + k3 + 1.0) + k3
}

/// Hash 3D coords to random float 0.0..=1.0
#[allow(dead_code)]
fn simple_hash(grid: Vec3) -> f32 {
    let (x, y, z) = (grid.x, grid.y, grid.z);
    let h = x * 37476.1393 + y * 668.265263 + z;
    let hh = (h.fract() * 1274.64) * (h.floor() / 6177.1);
    hh.fract() * 0.5
}

/// Thanks to Inigo Quilez (iquilezles.org)
fn grid_sphere(grid: Vec3, frac: Vec3, corner: Vec3) -> f32 {
    let radius = 0.5 * simple_hash(grid);

    return (frac - corner).length() - radius;
}

pub fn random_sphere_grid(p: Vec3) -> f32 {
    let grid = Vec3::new(p.x.floor(), p.y.floor(), p.z.floor());
    let fract = Vec3::new(p.x.fract(), p.y.fract(), p.z.fract());

    grid_sphere(grid, fract, Vec3::new(0.0, 0.0, 0.0))
        .min(grid_sphere(grid, fract, Vec3::new(0.0, 0.0, 1.0)))
        .min(
            grid_sphere(grid, fract, Vec3::new(0.0, 1.0, 0.0)).min(grid_sphere(
                grid,
                fract,
                Vec3::new(0.0, 1.0, 1.0),
            )),
        )
        .min(
            grid_sphere(grid, fract, Vec3::new(1.0, 0.0, 0.0))
                .min(grid_sphere(grid, fract, Vec3::new(1.0, 0.0, 1.0)))
                .min(
                    grid_sphere(grid, fract, Vec3::new(1.0, 1.0, 0.0)).min(grid_sphere(
                        grid,
                        fract,
                        Vec3::new(1.0, 1.0, 1.0),
                    )),
                ),
        )
}

// polynomial smooth min
fn smooth_min(a: f32, b: f32, k: f32) -> f32 {
    let h = (k - (a - b).abs()).max(0.0) / k;

    a.min(b) - h * h * k * (1.0 / 4.0)
}

// polynomial smooth max
fn smooth_max(a: f32, b: f32, k: f32) -> f32 {
    let h = (k - (b - a).abs()).max(0.0) / k;

    -(-a.min(-b) - h * h * k * (1.0 / 4.0))
}

/// sdBase(), an infinite grid of spheres with random radius
/// Once we have the base SDF sdBase(), we can start using it in our additive fractal construction of fBM with the redefined "addition" described above:
pub fn sphere_fbm_sdf(p: Vec3, mut d: f32) -> f32 {
    let mut scale = 1.0;
    let mut p = p.clone();
    for _ in 0..4 {
        // evaluate new octave
        let mut n = scale * random_sphere_grid(p);

        // add octave
        n = smooth_max(n, d - 0.1 * scale, 0.3 * scale);
        d = smooth_min(n, d, 0.3 * scale);

        // prep next octave
        p = Vec3::new(
            1.60 * p.y + 1.20 * p.z,
            -1.60 * p.x + 0.72 * p.y - 0.96 * p.z,
            -1.20 * p.x - 0.96 * p.y - 1.28 * p.z,
        ) * p;
        scale = 0.5 * scale;
    }

    d
}
