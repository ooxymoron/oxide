pub use std::{
    ffi::{c_char, CStr},
    mem::{size_of, transmute},
};

use libc::c_void;

use crate::{
    cfn,
    error::{OxideError, OxideResult},
    math::{
        angles::Angles, get_corners, vector::{Vector3, Vector4}
    },
};

use super::{entity::Entity, model_render::BoneMatrix, WithVmt};

pub type ModelInfo = WithVmt<VMTModelInfo>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HitboxSet {
    sznameindex: i32,
    numhitboxes: i32,
    hitboxindex: i32,
}

impl HitboxSet {
    pub fn get_hitbox(&self, id: HitboxId) -> OxideResult<&Hitbox> {
        let ptr = (self as *const _ as i64
            + self.hitboxindex as i64
            + size_of::<Hitbox>() as i64 * id as i64) as *const Hitbox;
        if ptr.is_null() {
            return Err(OxideError::new("could not get hitbox"));
        }
        return Ok(unsafe { transmute(ptr) });
    }
}

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
    pub id: HitboxId,
    pub group: i32,
    pub min: Vector3,
    pub max: Vector3,
    pub nameindex: i32,
    pub owner: &'static Entity,
}

impl HitboxWrapper {
    pub fn center(&self) -> OxideResult<Vector3> {
        let corners = self.corners()?;
        Ok((corners[0] + corners[7]) / 2.0)
    }
    pub fn get_pos(&self) -> OxideResult<(Vector3, Angles)> {
        let pos = Vector3::new(self.bone[0][3], self.bone[1][3], self.bone[2][3]);
        let angle = Vector3::new(self.bone[0][0], self.bone[0][1], self.bone[0][2]).angle();

        Ok((pos, angle))
    }
    pub fn corners(&self) -> OxideResult<[Vector3; 8]> {
        let (pos, rotation) = self.get_pos()?;
        Ok(get_corners(&pos, &rotation, &self.min, &self.max))
    }
    pub fn scaled(&self, scale: f32) -> HitboxWrapper {
        let mut hitbox = self.clone();
        hitbox.min *= scale;
        hitbox.max *= scale;
        hitbox
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitboxId {
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

impl HitboxId {
    pub fn body() -> Vec<HitboxId> {
        Self::all()[1..].to_vec()
    }
    pub fn all() -> Vec<HitboxId> {
        (0..=17)
            .map(|x| unsafe { transmute(x) })
            .collect::<Vec<HitboxId>>()
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Model {
    pub handle: &'static c_void,
    pub name: *const c_char,
    pub load_flags: i32,
    pub server_count: i32,
    pub r#type: i32,
    pub flags: i32,
    pub vec_mins: Vector3,
    pub vec_maxs: Vector3,
    pub radius: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct StudioHdr {
    pub id: i32,
    pub version: i32,
    pub checksum: i32,
    pub name: [c_char; 64],
    pub length: i32,
    pub eyeposition: Vector3,
    pub illumposition: Vector3,
    pub hull_min: Vector3,
    pub hull_max: Vector3,
    pub view_bbmin: Vector3,
    pub view_bbmax: Vector3,
    pub flags: i32,
    pub numbones: i32,
    pub boneindex: i32,
    pub numbonecontrollers: i32,
    pub bonecontrollerindex: i32,
    pub numhitboxsets: i32,
    pub hitboxsetindex: i32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Bone {
    sznameindex: i32,
    parent: i32,
    bonecontroller: [i32; 6],
    pos: Vector3,
    quat: Vector4,
    rot: Vector3,
    posscale: Vector3,
    rotscale: Vector3,
    pose_to_bone: BoneMatrix,
    alignment: Vector4,
    flags: i32,
    proctype: i32,
    procindex: i32,
    physicsbone: i32,
    surfacepropidx: i32,
    contents: i32,
    unused: [i32; 8],
}
impl StudioHdr {
    pub unsafe fn bone(&self, i: i32) -> Option<&Bone> {
        if i >= self.numbones {
            return None;
        }

        Some(&*(((self as *const _ as i32) + self.boneindex + i) as *const Bone))
    }

    pub unsafe fn get_hitbox_set(&self, i: i32) -> Option<&HitboxSet> {
        if i >= self.numhitboxsets {
            return None;
        }

        Some(
            &*((self as *const _ as i64
                + self.hitboxsetindex as i64
                + i as i64 * size_of::<HitboxSet>() as i64) as *const HitboxSet),
        )
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTModelInfo {
    _pad1: [isize; 3],
    pub get_model_index: cfn!(i32, &'static ModelInfo, &CStr),
    _pad2: [isize; 25],
    pub get_studio_model: cfn!(*const StudioHdr, *const ModelInfo, *const Model),
}
