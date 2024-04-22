pub use std::{
    ffi::{c_char, CStr},
    mem::{size_of, transmute},
};

use libc::c_void;

use crate::{
    cfn,
    error::{OxideError, OxideResult},
    math::{
        get_corners,
        vector::{Vector3, Vector4},
    },
    o, vmt_call,
};

use super::{
    entity::{Entity, MAX_STUDIO_BONES},
    model_render::Matrix3x4,
    WithVmt,
};

pub type ModelInfo = WithVmt<VMTModelInfo>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HitboxSet {
    sznameindex: i32,
    numhitboxes: i32,
    hitboxindex: i32,
}

impl HitboxSet {
    pub unsafe fn get_hitbox(&self, id: HitboxId) -> Option<Hitbox> {
        let ptr = (self as *const _ as i64
            + self.hitboxindex as i64
            + size_of::<Hitbox>() as i64 * id as i64) as *const Hitbox;
        if ptr.is_null() {
            return None;
        }
        Some(ptr.read_unaligned())
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Hitbox {
    pub bone: u32,
    pub group: i32,
    pub min: Vector3,
    pub max: Vector3,
    pub hitboxnameindex: i32,
    unused: [i32; 8],
}

impl Hitbox {
    pub fn center(&self, ent: &Entity) -> OxideResult<Vector3> {
        let corners = self.corners(ent)?;
        Ok((corners[0] + corners[7]) / 2.0)
    }
    pub fn get_bone_pos(&self, ent: &Entity) -> OxideResult<(Vector3, [Vector3; 3])> {
        if self.bone as usize >= MAX_STUDIO_BONES {
            return Err(OxideError::new("invalid bone index"));
        };

        let bones = o!()
            .last_entity_cache
            .clone()
            .unwrap()
            .get_bones(vmt_call!(ent, get_index))?;

        let bone = bones[self.bone as usize].clone();
        let pos = Vector3::new(bone[0][3], bone[1][3], bone[2][3]);
        let angle = [
            Vector3::new(bone[0][0], bone[0][1], bone[0][2]),
            Vector3::new(bone[1][0], bone[1][1], bone[1][2]),
            Vector3::new(bone[2][0], bone[2][1], bone[2][2]),
        ];

        Ok((pos, angle))
    }
    pub fn corners(&self, ent: &Entity) -> OxideResult<[Vector3; 8]> {
        let (pos, rotation) = self.get_bone_pos(ent)?;
        Ok(get_corners(&pos, &rotation, &self.min, &self.max))
    }
    pub fn scaled(&self, scale: f32) -> Hitbox {
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
    pose_to_bone: Matrix3x4,
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
            &*((self as *const _ as i64 + self.hitboxsetindex as i64 + i as i64 * size_of::<HitboxSet>() as i64) 
                as *const HitboxSet),
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
