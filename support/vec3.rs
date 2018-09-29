use std::f32;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;

#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f32) -> Vec3 {
        Vec3(self.0 * f, self.1 * f, self.2 * f)
    }
}

pub fn norm(v: &Vec3) -> Vec3 {
    let len = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt();
    Vec3(v.0 / len, v.1 / len, v.2 / len)
}
