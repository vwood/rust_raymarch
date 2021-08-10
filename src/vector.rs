use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn from_float(value: f32) -> Vec3 {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self: &Vec3, b: &Vec3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn cross(self: &Vec3, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl From<Vec3> for (f32, f32, f32) {
    fn from(p: Vec3) -> Self {
        (p.x, p.y, p.z)
    }
}

impl From<Vec3> for (i32, i32, i32) {
    fn from(p: Vec3) -> Self {
        (p.x as i32, p.y as i32, p.z as i32)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Vec3 { x, y, z }
    }
}

impl From<(i32, i32, i32)> for Vec3 {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Vec3 {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }
}

macro_rules! vec3_operation {
    ($operation:ident, $op_fn:ident, $op: tt) => {
        impl $operation<f32> for Vec3 {
            type Output = Vec3;
            fn $op_fn(self, other: f32) -> Vec3 {
                Vec3 {
                    x: self.x $op other,
                    y: self.y $op other,
                    z: self.z $op other,
                }
            }
        }

        impl $operation<Vec3> for f32 {
            type Output = Vec3;
            fn $op_fn(self, other: Vec3) -> Vec3 {
                Vec3 {
                    x: self $op other.x,
                    y: self $op other.y,
                    z: self $op other.z,
                }
            }
        }

        impl $operation<&Vec3> for f32 {
            type Output = Vec3;
            fn $op_fn(self, other: &Vec3) -> Vec3 {
                Vec3 {
                    x: self $op other.x,
                    y: self $op other.y,
                    z: self $op other.z,
                }
            }
        }

        impl $operation<Vec3> for Vec3 {
            type Output = Vec3;
            fn $op_fn(self, other: Vec3) -> Vec3 {
                Vec3 {
                    x: self.x $op other.x,
                    y: self.y $op other.y,
                    z: self.z $op other.z,
                }
            }
        }
    };
}

vec3_operation!(Div, div, /);
vec3_operation!(Mul, mul, *);
vec3_operation!(Add, add, +);
vec3_operation!(Sub, sub, -);
