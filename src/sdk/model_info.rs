pub use std::{
    ffi::{c_char, CStr},
    mem::{size_of, transmute},
};

use libc::c_void;

use crate::{
    vmt_call, cfn,
    error::{OxideError, OxideResult},
    math::{
        get_corners,
        vector::{Vector3, Vector4},
    },
    o,
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
    sznameindex: usize,
    numhitboxes: usize,
    hitboxindex: usize,
}

impl HitboxSet {
    pub unsafe fn get_hitbox(&self, id: HitboxId) -> Option<Hitbox> {
        let ptr = (self as *const _ as usize + self.hitboxindex + size_of::<Hitbox>() * id as usize)
            as *const Hitbox;
        if ptr.is_null() {
            return None;
        }
        Some(ptr.read_unaligned())
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Hitbox {
    pub bone: usize,
    pub group: usize,
    pub min: Vector3,
    pub max: Vector3,
    pub hitboxnameindex: usize,
    unused: [usize; 8],
}

impl Hitbox {
    pub fn center(&self, ent: &Entity) -> OxideResult<Vector3> {
        let corners = self.corners(ent)?;
        Ok((corners[0] + corners[7]) / 2.0)
    }
    pub fn get_bone_pos(&self, ent: &Entity) -> OxideResult<(Vector3, [Vector3; 3])> {
        if self.bone >= MAX_STUDIO_BONES {
            return Err(OxideError::new("invalid bone index"));
        };

        let bones = o!()
            .last_entity_cache
            .clone()
            .unwrap()
            .get_bones(vmt_call!(ent, get_index))?;

        let bone = bones[self.bone].clone();
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
    pub load_flags: isize,
    pub server_count: isize,
    pub r#type: isize,
    pub flags: isize,
    pub vec_mins: Vector3,
    pub vec_maxs: Vector3,
    pub radius: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct StudioHdr {
    pub id: usize,
    pub version: usize,
    pub checksum: usize,
    pub name: [c_char; 64],
    pub length: usize,
    pub eyeposition: Vector3,
    pub illumposition: Vector3,
    pub hull_min: Vector3,
    pub hull_max: Vector3,
    pub view_bbmin: Vector3,
    pub view_bbmax: Vector3,
    pub flags: usize,
    pub numbones: usize,
    pub boneindex: usize,
    pub numbonecontrollers: usize,
    pub bonecontrollerindex: usize,
    pub numhitboxsets: usize,
    pub hitboxsetindex: usize,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Bone {
    sznameindex: usize,
    parent: usize,
    bonecontroller: [usize; 6],
    pos: Vector3,
    quat: Vector4,
    rot: Vector3,
    posscale: Vector3,
    rotscale: Vector3,
    pose_to_bone: Matrix3x4,
    alignment: Vector4,
    flags: usize,
    proctype: usize,
    procindex: usize,
    physicsbone: usize,
    surfacepropidx: usize,
    contents: usize,
    unused: [usize; 8],
}
impl StudioHdr {
    pub unsafe fn bone(&self, i: usize) -> Option<&Bone> {
        if i >= self.numbones {
            return None;
        }

        Some(&*(((self as *const _ as usize) + self.boneindex + i) as *const Bone))
    }

    pub unsafe fn get_hitbox_set(&self, i: usize) -> Option<&HitboxSet> {
        if i >= self.numhitboxsets {
            return None;
        }

        Some(
            &*((self as *const _ as usize + self.hitboxsetindex + i * size_of::<HitboxSet>())
                as *const HitboxSet),
        )
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTModelInfo {
    _pad1: [u8; 4 * 3],
    pub get_model_index: cfn!(isize, &'static ModelInfo, &CStr),
    _pad2: [u8; 4 * 25],
    pub get_studio_model: cfn!(*const StudioHdr, *const ModelInfo, *const Model),
}
