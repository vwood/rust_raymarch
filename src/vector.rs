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

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
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
