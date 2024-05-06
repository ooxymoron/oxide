use std::{
    f32::consts::PI,
    ops::{Add, AddAssign, Div, DivAssign, Mul, Sub, SubAssign},
};

use crate::sdk::interfaces::engine_trace::VectorAligned;

use super::angles::{RotationVectors, Angles};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Add<f32> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}
impl Sub<f32> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl AddAssign<f32> for Vector3 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}
impl SubAssign<f32> for Vector3 {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}
impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl std::ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl std::ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
    pub fn dot(&self, vec: &Vector3) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }
    pub fn len2d(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(mut self) -> Vector3 {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
        self
    }
    pub fn angle(&self) -> Angles {
        Angles {
            pitch: (self.z/self.len2d()).asin() / PI * 180f32,
            yaw: -(self.x.atan2(self.y) / PI * 180f32),
            roll: 0.0,
        }
    }
    pub fn zeroed() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
    pub fn rotate(&self, rotation: &RotationVectors) -> Vector3 {
        Vector3::new(
            self.dot(&rotation.forward),
            self.dot(&rotation.right),
            self.dot(&rotation.up),
        )
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
    pub fn dot(&self, vec: Vector2) -> f32 {
        self.x * vec.x + self.y * vec.y
    }
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn zeroed() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }
}
impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}
impl Add<f32> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: f32) -> Vector2 {
        Vector2::new(self.x + rhs, self.y + rhs)
    }
}

impl AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        *self = *self + rhs;
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Vector2 {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3::new(0f32, 0f32, 0f32)
    }
}

impl Into<VectorAligned> for Vector3 {
    fn into(self) -> VectorAligned {
        VectorAligned::new(self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub t: f32,
}
