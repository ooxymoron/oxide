use std::ops::{Add, AddAssign, Sub, SubAssign};

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

        let mut vecs = AngleVectors{forward:Vector3::zeroed(), right:Vector3::zeroed(), up:Vector3::zeroed()};
        vecs.forward.x = cp * cy;
        vecs.right.x = cp * sy;
        vecs.up.x = -sp;

        let crcy = cr * cy;
        let crsy = cr * sy;
        let srcy = sr * cy;
        let srsy = sr * sy;
        vecs.forward.y = sp * srcy - crsy;
        vecs.right.y = sp * srsy + crcy;
        vecs.up.y = sr * cp;

        vecs.forward.z = sp * crcy + srsy;
        vecs.right.z = sp * crsy - srcy;
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
