use std::ffi::c_char;

use libc::c_short;

use crate::math::{angles::Angles, vector::Vector3};

use super::{interfaces::cvar::Color, EntHandle};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EffectData {
    pub origin: Vector3,
    pub start: Vector3,
    pub normal: Vector3,
    pub angles: Angles,
    pub flags: u32,
    pub entity: EntHandle,
    pub scale: f32,
    pub magnitude: f32,
    pub radius: f32,
    pub attachment_index: u32,
    pub surface_prop: c_short,
    pub material: u32,
    pub damage_type: u32,
    pub hitbox: u32,
    pub color: c_char,
}
