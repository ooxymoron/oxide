use std::mem::transmute;

use derivative::Derivative;

use crate::error::OxideResult;

use super::model_info::HitboxId;

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub enum ObjectLevel {
    BUILDING,
    ONE,
    TWO,
    THREE,
}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Object {
    #[derivative(Debug = "ignore")]
    _pad1: [u8; 0xEE4],
    pub level: ObjectLevel,
    #[derivative(Debug = "ignore")]
    _pad2: [u8; 0x55],
    pub mini: bool,
}

impl Object {
    pub fn as_sentry(&mut self) -> OxideResult<&'static mut Sentry> {
        return Ok(unsafe { transmute(self) });
    }
}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Sentry {}

impl Sentry {
    pub fn as_object(&mut self) -> OxideResult<&'static mut Object> {
        return Ok(unsafe { transmute(self) });
    }
    pub fn get_hitbox_ids(&mut self) -> Vec<HitboxId> {
        let obj = self.as_object().unwrap();
        if obj.mini {
            return vec![HitboxId::Head]
        }
        match obj.level {
            ObjectLevel::BUILDING | ObjectLevel::ONE=> vec![HitboxId::Head, HitboxId::Spine1],
            ObjectLevel::TWO => vec![
                HitboxId::Head,
                HitboxId::Spine0,
                HitboxId::LeftHand,
                HitboxId::LeftLowerArm,
            ],
            ObjectLevel::THREE => vec![
                HitboxId::Head,
                HitboxId::Spine0,
                HitboxId::Spine3,
                HitboxId::LeftHip,
                HitboxId::RightUpperArm,
            ],
        }
    }
}
