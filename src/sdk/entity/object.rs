use std::mem::transmute;

use derivative::Derivative;

use crate::{define_netvar, error::OxideResult, netvars::HasNetvars};

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
        if *obj.get_mini() {
            return vec![HitboxId::Head]
        }
        match *obj.get_level() {
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
impl HasNetvars for Object {
    fn get_class_name() -> String {
        "CBaseObject".to_string()
    }
}
impl Object {
    define_netvar!(get_level, ["m_iUpgradeLevel"], ObjectLevel);
    define_netvar!(get_mini, ["m_bMiniBuilding"], bool);
}
