use std::mem::transmute;

use derivative::Derivative;

use crate::{
    vmt_call,
    draw::colors::{BLUE, RED},
    error::{OxideError, OxideResult},
    interface,
    math::{angles::Angles, vector::Vector3},
    o,
};

use self::{
    attribute_manager::HasAttributes, flags::Flags, model_info::{Hitbox, HitboxId}, model_render::Matrix3x4, networkable::ClassId, object::Object, pipe::Pipe, player::Player
};

use super::*;

use super::{
    collideable::Collideable, model_render::Renderable, networkable::Networkable, weapon::Weapon,
};

pub mod flags;
pub mod object;
pub mod paint;
pub mod pipe;
pub mod player;
pub mod weapon;

pub const MAX_STUDIO_BONES: usize = 128;
pub type Bones = [Matrix3x4; MAX_STUDIO_BONES];
pub const HITBOX_SET: usize = 0;

#[repr(C)]
#[derive(Debug, Clone)]
pub enum BoneMask {
    Anything = 0x0007FF00,
    Hitbox = 0x00000100,
    Attachment = 0x00000200,
    VertexMask = 0x0003FC00,
    VertexLod0 = 0x00000400,
    VertexLod1 = 0x00000800,
    VertexLod2 = 0x00001000,
    VertexLod3 = 0x00002000,
    VertexLod4 = 0x00004000,
    VertexLod5 = 0x00008000,
    VertexLod6 = 0x00010000,
    VertexLod7 = 0x00020000,
    BoneMerge = 0x00040000,
}

#[repr(C)]
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct VMTEntity {
    #[derivative(Debug = "ignore")]
    _pad1: [u32; 4],
    pub get_collideable: cfn!(&Collideable, &Entity),
    #[derivative(Debug = "ignore")]
    _pad2: [u32; 6],
    pub get_abs_origin: cfn!(*const Vector3, *const Entity),
    pub get_abs_angles: cfn!(&'static Angles, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad3: [u32; 66],
    pub get_index: cfn!(isize, &Entity),
    #[derivative(Debug = "ignore")]
    _pad4: [u32; 26],
    pub world_space_center: cfn!(&Vector3, &Entity),
    #[derivative(Debug = "ignore")]
    _pad5: [u32; 10],
    pub get_team_number: cfn!(Team, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad6: [u32; 34],
    pub get_health: cfn!(isize, &Entity),
    pub get_max_health: cfn!(isize, &Entity),
    #[derivative(Debug = "ignore")]
    _pad7: [u32; 29],
    pub is_alive: cfn!(bool, *const Entity),
    pub is_player: cfn!(bool, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad8: [u32; 2],
    pub is_npc: cfn!(bool, &Entity),
    #[derivative(Debug = "ignore")]
    _pad9: [u32; 2],
    pub is_weapon: cfn!(bool, &Entity),
    pub get_weapon2: cfn!(*mut u8, &Entity),
    #[derivative(Debug = "ignore")]
    _pad10: [u32; 2],
    pub eye_position: cfn!(Vector3, *const Entity),
    pub eye_angles: cfn!(Angles, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad11: [u32; 12],
    pub third_person_switch: cfn!((), &Entity, bool),
    #[derivative(Debug = "ignore")]
    _pad12: [u32; 82],
    pub get_weapon: cfn!(&'static mut Weapon, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad13: [u32; 10],
    pub get_shoot_pos: cfn!(Vector3, &Entity),
    #[derivative(Debug = "ignore")]
    _pad14: [u32; 6],
    pub get_observer_mode: cfn!(ObserverMode, &Entity),
    pub get_observer_target: cfn!(&Entity, &Entity),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObserverMode {
    None = 0,
    Deathcam,
    Freezecam,
    Fixed,
    InEye,
    Chase,
    Poi,
    Roaming,
}

impl ObserverMode {
    pub fn to_string(&self) -> &str {
        match self {
            ObserverMode::None => "invalid",
            ObserverMode::Deathcam => "death",
            ObserverMode::Freezecam => "freeze",
            ObserverMode::Fixed => "fixed",
            ObserverMode::InEye => "1st",
            ObserverMode::Chase => "3rd",
            ObserverMode::Poi => "poi",
            ObserverMode::Roaming => "free",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaterLevel {
    NotInWater,
    Feet,
    Waist,
    Eyes,
}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Entity {
    pub vmt: *mut VMTEntity,
    #[derivative(Debug = "ignore")]
    _pad11: [*const u8; 0xF],
    pub attributes: &'static HasAttributes,
    #[derivative(Debug = "ignore")]
    _pad1: [u8; 0x3c],
    pub model_idx: isize,
    #[derivative(Debug = "ignore")]
    _pad2: [u8; 0x8C],
    pub velocity: Vector3,
    #[derivative(Debug = "ignore")]
    _pad3: [u8; 0x7C],
    pub water_level: WaterLevel,
    #[derivative(Debug = "ignore")]
    _pad4: [u8; 0x1B8],
    pub vec_origin: Vector3,
    #[derivative(Debug = "ignore")]
    _pad5: [u8; 0xC],
    pub flags: Flags,
}

impl_has_vmt!(Entity, VMTEntity);

impl Entity {
    pub fn as_renderable(&self) -> &mut Renderable {
        unsafe { transmute(transmute::<&Self, usize>(self) + 4) }
    }
    pub fn as_networkable(&self) -> &mut Networkable {
        unsafe { transmute(transmute::<&Self, usize>(self) + 8) }
    }

    //todo make this shit a result type
    pub fn get_hitbox(&mut self, hitbox_id: HitboxId) -> Option<Hitbox> {
        unsafe {
            let rend = self.as_renderable();

            let model = vmt_call!(rend, get_model);
            let studio_model = &*vmt_call!(interface!(model_info), get_studio_model, model);

            let Some(hitbox_set) = studio_model.get_hitbox_set(HITBOX_SET) else {
                return None;
            };
            let Some(hitbox) = hitbox_set.get_hitbox(hitbox_id) else {
                return None;
            };
            Some(hitbox)
        }
    }
}

impl Entity {
    pub fn get_local() -> OxideResult<&'static mut Player> {
        let id = vmt_call!(interface!(base_engine), get_local_player);
        let Some(ent) = Self::get_ent(id) else {
            return Err(OxideError::new("plocal is none"))
        };
        return ent.as_player();
    }
}
impl Entity {
    pub fn get_ent(id: isize) -> Option<&'static mut Entity> {
        let ent = vmt_call!(interface!(entity_list), get_client_entity, id);
        if ent.is_null() {
            return None;
        }
        unsafe { Some(&mut *ent) }
    }
    pub fn as_player(&mut self) -> OxideResult<&'static mut Player> {
        if !vmt_call!(self, is_player) {
            return Err(OxideError::new("not a player"));
        };
        return Ok(unsafe { transmute(self) });
    }
    pub fn as_pipe(&mut self) -> OxideResult<&'static mut Pipe> {
        if !matches!(
            self.as_networkable().get_client_class().class_id,
            ClassId::CTFGrenadePipebombProjectile
        ) {
            return Err(OxideError::new("not a pipe"));
        };
        return Ok(unsafe { transmute(self) });
    }
    pub fn as_object(&mut self) -> OxideResult<&'static mut Object> {
        if !matches!(
            self.as_networkable().get_client_class().class_id,
            ClassId::CObjectSentrygun
        ) {
            return Err(OxideError::new("not a object"));
        };
        return Ok(unsafe { transmute(self) });
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Team {
    Red = 2,
    Blue = 3,
}

impl Team {
    pub fn color(&self) -> usize {
        match self {
            Team::Red => RED,
            Team::Blue => BLUE,
        }
    }
}

impl PartialEq for &Entity {
    fn eq(&self, other: &Self) -> bool {
        let self_id = vmt_call!(self.as_networkable(), get_index);
        let other_id = vmt_call!(other.as_networkable(), get_index);
        self_id == other_id
    }
}
