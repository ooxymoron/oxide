use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use super::{dtr, vector3::Vector3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewAngles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}
impl ViewAngles {
    pub fn to_angles(&self) -> Angles {
        Angles {
            pitch: self.pitch,
            yaw: self.yaw,
            roll: self.roll,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RotationVectors {
    pub forward: Vector3,
    pub right: Vector3,
    pub up: Vector3,
}

impl Angles {
    pub fn new(yaw: f32, pitch: f32, roll: f32) -> Angles {
        Angles { pitch, yaw, roll }
    }
    pub fn zeroed() -> Angles {
        Angles::new(0.0, 0.0, 0.0)
    }
    pub fn to_vectors(&self) -> RotationVectors {
        let (sy, cy) = dtr(self.yaw).sin_cos();
        let (sp, cp) = dtr(self.pitch).sin_cos();
        let (sr, cr) = dtr(self.roll).sin_cos();
        RotationVectors {
            forward: Vector3::new(cp * cy, cp * sy, -sp),
            right: Vector3::new(sr * sp * cy - cr * sy, sr * sp * sy + cr * cy, sr * cp),
            up: Vector3::new(cr * sp * cy + sr * sy, cr * sp * sy - sr * cy, cr * cp),
        }
    }
    pub fn to_view_angles(&self) -> ViewAngles {
        ViewAngles {
            pitch: self.pitch.clamp(-89.0, 89.0),
            yaw: ((self.yaw + 180.0) % 360.0 - 180.0),
            roll: self.roll,
        }
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
