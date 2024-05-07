pub use std::{
    ffi::{c_char, CStr},
    mem::{size_of, transmute},
};

use libc::c_void;

use crate::{
    cfn,
    error::{OxideError, OxideResult},
    math::{vector3::Vector3, vector4::Vector4},
};

use super::{
    entity::hitbox::{Hitbox, HitboxId},
    model_render::BoneMatrix,
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

    pub fn get_hitbox_set(&self, i: i32) -> Option<&HitboxSet> {
        unsafe {
            if i >= self.numhitboxsets {
                return None;
            }

            Some(
                &*((self as *const _ as i64
                    + self.hitboxsetindex as i64
                    + i as i64 * size_of::<HitboxSet>() as i64)
                    as *const HitboxSet),
            )
        }
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
