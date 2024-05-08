use std::mem::transmute;

use crate::{
    error::OxideResult,
    math::{angles::RotationVectors, get_corners, vector3::Vector3},
};

use super::interfaces::{model_render::BoneMatrix, Entity};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Hitbox {
    pub bone: u32,
    pub group: i32,
    pub min: Vector3,
    pub max: Vector3,
    pub nameindex: i32,
    unused: [i32; 8],
}

#[derive(Debug, Clone, Copy)]
pub struct HitboxWrapper {
    pub bone: BoneMatrix,
    pub id: usize,
    pub group: i32,
    pub min: Vector3,
    pub max: Vector3,
    pub nameindex: i32,
    pub owner: &'static Entity,
    pub corner_cache: Option<[Vector3; 8]>,
}

impl HitboxWrapper {
    pub fn center(&mut self) -> OxideResult<Vector3> {
        let corners = self.corners().unwrap();
        Ok((corners[0] + corners[7]) / 2.0)
    }
    pub fn get_pos(&self) -> OxideResult<(Vector3, RotationVectors)> {
        let pos = Vector3::new(self.bone[0][3], self.bone[1][3], self.bone[2][3]);
        let angle = RotationVectors {
            forward: Vector3::new(self.bone[0][0], self.bone[0][1], self.bone[0][2]),
            right: Vector3::new(self.bone[1][0], self.bone[1][1], self.bone[1][2]),
            up: Vector3::new(self.bone[2][0], self.bone[2][1], self.bone[2][2]),
        };

        Ok((pos, angle))
    }
    pub fn corners(&mut self) -> OxideResult<[Vector3; 8]> {
        if let Some(corners) = self.corner_cache {
            return Ok(corners);
        }
        let (pos, rotation) = self.get_pos()?;
        let corners = get_corners(&pos, &rotation, &self.min, &self.max);
        self.corner_cache = Some(corners);
        Ok(corners)
    }
    pub fn scaled(&self, scale: f32) -> HitboxWrapper {
        let mut hitbox = self.clone();
        hitbox.min *= scale;
        hitbox.max *= scale;
        hitbox
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerHitboxId {
    Head,
    Pelvis,
    Spine0,
    Spine1,
    Spine2,
    Spine3,
    LeftUpperArm,
    LeftLowerArm,
    LeftHand,
    RightUpperArm,
    RightLowerArm,
    RightHand,
    LeftHip,
    LeftKnee,
    LeftFoot,
    RightHip,
    RightKnee,
    RightFoot,
}

impl PlayerHitboxId {
    pub fn body() -> Vec<PlayerHitboxId> {
        Self::all()[1..].to_vec()
    }
    pub fn all() -> Vec<PlayerHitboxId> {
        (0..=17)
            .map(|x| unsafe { transmute(x) })
            .collect::<Vec<PlayerHitboxId>>()
    }
}
impl From<usize> for PlayerHitboxId {
    fn from(value: usize) -> Self {
        unsafe { transmute(value as u32) }
    }
}
impl Into<usize> for PlayerHitboxId {
    fn into(self) -> usize {
        (unsafe { transmute::<_, u32>(self) }) as usize
    }
}
