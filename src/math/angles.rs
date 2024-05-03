use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use super::{dtr, vector::Vector3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AngleVectors {
    pub forward: Vector3,
    pub right: Vector3,
    pub up: Vector3,
}

impl Angles {
    pub fn new(yaw: f32, pitch: f32, roll: f32) -> Angles {
        Angles { pitch, yaw, roll }
    }
    pub fn to_vectors(&self) -> AngleVectors {
        let sy = dtr(self.yaw).sin();
        let cy = dtr(self.yaw).cos();
        let sp = dtr(self.pitch).sin();
        let cp = dtr(self.pitch).cos();
        let sr = dtr(self.roll).sin();
        let cr = dtr(self.roll).cos();

        let mut vecs = AngleVectors {
            forward: Vector3::zeroed(),
            right: Vector3::zeroed(),
            up: Vector3::zeroed(),
        };
        vecs.forward.x = cp * cy;
        vecs.forward.y = cp * sy;
        vecs.forward.z = -sp;

        vecs.right.x = sr * sp * cy - cr * sy;
        vecs.right.y = sr * sp * sy + cr * cy;
        vecs.right.z = sr * cp;

        vecs.up.x = cr * sp * cy + sr * sy;
        vecs.up.y = cr * sp * cy - sr * cy;
        vecs.up.z = cr * cp;

        vecs
    }
}

impl Sub for Angles {
    type Output = Angles;

    fn sub(self, rhs: Self) -> Self::Output {
        Angles::new(
            self.pitch - rhs.pitch,
            self.yaw - rhs.yaw,
            self.roll - rhs.roll,
        )
    }
}
impl SubAssign for Angles {
    fn sub_assign(&mut self, rhs: Self) {
        self.pitch -= rhs.pitch;
        self.yaw -= rhs.yaw;
        self.roll -= rhs.roll;
    }
}

impl Add for Angles {
    type Output = Angles;

    fn add(self, rhs: Self) -> Self::Output {
        Angles::new(
            self.pitch + rhs.pitch,
            self.yaw + rhs.yaw,
            self.roll + rhs.roll,
        )
    }
}
impl AddAssign for Angles {
    fn add_assign(&mut self, rhs: Self) {
        self.pitch += rhs.pitch;
        self.yaw += rhs.yaw;
        self.roll += rhs.roll;
    }
}
impl Mul<f32> for Angles {
    type Output = Angles;

    fn mul(self, rhs: f32) -> Self::Output {
        Angles::new(self.pitch * rhs, self.yaw * rhs, self.roll * rhs)
    }
}
impl MulAssign<f32> for Angles {
    fn mul_assign(&mut self, rhs: f32) {
        self.pitch *= rhs;
        self.yaw *= rhs;
        self.roll *= rhs;
    }
}
