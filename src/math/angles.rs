use std::ops::Sub;

use super::{dtr, vector::Vector3};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Angles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Angles {
    pub fn new(yaw: f32, pitch: f32, roll: f32) -> Angles {
        Angles { pitch, yaw, roll }
    }
    pub fn to_vectors(&self) -> [Vector3; 3] {
        let sy = dtr(self.yaw).sin();
        let cy = dtr(self.yaw).cos();
        let sp = dtr(self.pitch).sin();
        let cp = dtr(self.pitch).cos();
        let sr = dtr(self.roll).sin();
        let cr = dtr(self.roll).cos();

        let mut vecs = [Vector3::zeroed(), Vector3::zeroed(), Vector3::zeroed()];
        vecs[0].x = cp * cy;
        vecs[1].x = cp * sy;
        vecs[2].x = -sp;

        let crcy = cr * cy;
        let crsy = cr * sy;
        let srcy = sr * cy;
        let srsy = sr * sy;
        vecs[0].y = sp * srcy - crsy;
        vecs[1].y = sp * srsy + crcy;
        vecs[2].y = sr * cp;

        vecs[0].z = sp * crcy + srsy;
        vecs[1].z = sp * crsy - srcy;
        vecs[2].z = cr * cp;

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
