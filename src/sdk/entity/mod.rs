use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::{mem::transmute, ptr::null};

use derivative::Derivative;
use std::ffi::CString;

use crate::draw::colors::FOREGROUND;
use crate::{
    define_netvar,
    draw::colors::{BLUE, RED},
    error::{OxideError, OxideResult},
    interface,
    math::{angles::Angles, vector3::Vector3},
    netvars::HasNetvars,
    o, vmt_call,
};

use self::hitbox::HitboxWrapper;
use self::{
    interfaces::model_info::StudioHdr, interfaces::model_render::BoneMatrix, networkable::ClassId,
    object::Object, pipe::PipeBomb, player::Player, weapon::Weapon,
};

use super::*;

use super::{
    collideable::Collideable, interfaces::model_render::Renderable, networkable::Networkable,
};

pub mod flags;
pub mod hitbox;
pub mod object;
pub mod pipe;
pub mod player;
pub mod weapon;

pub const MAX_STUDIO_BONES: usize = 128;
pub type Bones = [BoneMatrix; MAX_STUDIO_BONES];
pub const HITBOX_SET: i32 = 0;

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
    _pad1: [usize; 4],
    pub get_collideable: cfn!(&Collideable, &Entity),
    #[derivative(Debug = "ignore")]
    _pad2: [usize; 6],
    pub get_abs_origin: cfn!(*const Vector3, *const Entity),
    pub get_abs_angles: cfn!(&'static mut Angles, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad3: [usize; 66],
    pub get_index: cfn!(i32, &Entity),
    #[derivative(Debug = "ignore")]
    _pad4: [usize; 26],
    pub world_space_center: cfn!(&Vector3, &Entity),
    #[derivative(Debug = "ignore")]
    _pad5: [usize; 10],
    pub get_team_number: cfn!(Team, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad6: [usize; 34],
    pub get_health: cfn!(i32, &Entity),
    pub get_max_health: cfn!(i32, &Entity),
    #[derivative(Debug = "ignore")]
    _pad7: [usize; 29],
    pub is_alive: cfn!(bool, *const Entity),
    pub is_player: cfn!(bool, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad8: [usize; 2],
    pub is_npc: cfn!(bool, &Entity),
    #[derivative(Debug = "ignore")]
    _pad9: [usize; 2],
    pub is_weapon: cfn!(bool, &Entity),
    pub get_weapon2: cfn!(*mut u8, &Entity),
    #[derivative(Debug = "ignore")]
    _pad10: [usize; 2],
    pub eye_position: cfn!(Vector3, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad101: [usize; 1],
    #[derivative(Debug = "ignore")]
    _pad11: [usize; 12],
    pub third_person_switch: cfn!((), &Entity, bool),
    #[derivative(Debug = "ignore")]
    _pad12: [usize; 82],
    pub get_weapon: cfn!(&'static mut Weapon, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad13: [usize; 10],
    pub get_shoot_pos: cfn!(Vector3, &Entity),
    #[derivative(Debug = "ignore")]
    _pad14: [usize; 6],
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
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Entity {
    pub vmt: *mut VMTEntity,
}

impl_has_vmt!(Entity, VMTEntity);

impl Entity {
    pub fn as_renderable(&self) -> &mut Renderable {
        unsafe { transmute(transmute::<&Self, usize>(self) + 8) }
    }
    pub fn as_networkable(&self) -> &mut Networkable {
        unsafe { transmute(transmute::<&Self, usize>(self) + 16) }
    }
    pub fn priority(&mut self) -> Option<isize> {
        let p_local = Player::get_local().unwrap();
        let team = vmt_call!(self, get_team_number);
        let local_team = vmt_call!(p_local.as_ent(), get_team_number);
        if local_team == team {
            return None;
        }
        if let Ok(player) = self.as_player() {
            return player.priority();
        }
        Some(0)
    }
    pub fn get_float_attrib(&self, name: &str) -> Option<f32> {
        let name = CString::new(name).unwrap();
        let defualt_value = unsafe { transmute::<_, f32>(0b1111111111111111) };
        let val = (o!().util.get_float_attribute)(defualt_value, name.as_ptr(), self, null(), true);
        if val == defualt_value {
            return None;
        }
        Some(val)
    }
}

impl Entity {
    pub fn get_hitbox(&self, hitbox_id: usize) -> OxideResult<&mut HitboxWrapper> {
        Ok(self.get_hitboxes()?.get_mut(&hitbox_id).unwrap())
    }
    pub fn calculate_hitboxes(&self) -> OxideResult<HashMap<usize, HitboxWrapper>> {
        let rend = self.as_renderable();

        let bones = unsafe { MaybeUninit::zeroed().assume_init() };
        vmt_call!(
            rend,
            setup_bones,
            &bones,
            MAX_STUDIO_BONES as u32,
            BoneMask::Hitbox,
            o!().global_vars.curtime
        );
        let model = vmt_call!(rend, get_model);
        let studio_model = unsafe {
            transmute::<_, &StudioHdr>(vmt_call!(interface!(model_info), get_studio_model, model))
        };
        let hitbox_set = studio_model
            .get_hitbox_set(HITBOX_SET)
            .ok_or(OxideError::new("could not get hitboxes"))?;
        (0..hitbox_set.numhitboxes as usize)
            .into_iter()
            .map(|id| {
                let hitbox = hitbox_set.get_hitbox(id)?;
                Ok((
                    id,
                    HitboxWrapper {
                        id,
                        bone: bones[hitbox.bone as usize].clone(),
                        group: hitbox.group,
                        min: hitbox.min,
                        max: hitbox.max,
                        nameindex: hitbox.nameindex,
                        owner: unsafe { transmute(self) },
                        corner_cache: None,
                    },
                ))
            })
            .collect()
    }

    pub fn get_hitboxes(&self) -> OxideResult<&mut HashMap<usize, HitboxWrapper>> {
        Ok(o!()
            .last_entity_cache
            .as_mut()
            .unwrap()
            .get_hitboxes(vmt_call!(self, get_index))?)
    }
}

impl Entity {
    pub fn get_ent(id: i32) -> Option<&'static mut Entity> {
        let ent = vmt_call!(interface!(entity_list), get_client_entity, id);
        if ent.is_null() {
            return None;
        }
        unsafe { Some(&mut *ent) }
    }
    pub fn get_ent_from_handle(handle: EntHandle) -> Option<&'static mut Entity> {
        let ent = vmt_call!(
            interface!(entity_list),
            get_client_entity,
            handle.0 & ENT_ENTRY_MASK
        );
        if ent.is_null() {
            return None;
        }
        unsafe { Some(&mut *ent) }
    }
    pub fn as_player(&mut self) -> OxideResult<&'static mut Player> {
        if !matches!(
            self.as_networkable().get_client_class().class_id,
            ClassId::CTFPlayer
        ) {
            return Err(OxideError::new("not a player"));
        };
        return Ok(unsafe { transmute(self) });
    }
    pub fn as_pipe(&mut self) -> OxideResult<&'static mut PipeBomb> {
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
    pub fn as_weapon(&mut self) -> OxideResult<&'static mut Weapon> {
        //TODO: check if extends weaon
        return Ok(unsafe { transmute(self) });
    }
}

#[repr(C)]
#[derive(Debug, Clone,Copy, PartialEq, Eq)]
pub enum Team {
    None = 1,
    Red = 2,
    Blue = 3,
}

impl Team {
    pub fn color(&self) -> usize {
        match self {
            Team::None => FOREGROUND,
            Team::Red => RED,
            Team::Blue => BLUE,
        }
    }
}

impl From<i32> for Team {
    fn from(value: i32) -> Self {
        unsafe { transmute(value) }
    }
}
impl  Team {
    pub fn as_i32(self) -> i32 {
        unsafe { transmute(self) }
    }
}

impl PartialEq for &Entity {
    fn eq(&self, other: &Self) -> bool {
        *self as *const _ as usize == *other as *const _ as usize
    }
}
impl PartialEq for &mut Entity {
    fn eq(&self, other: &Self) -> bool {
        *self as *const _ as usize == *other as *const _ as usize
    }
}

impl HasNetvars for Entity {
    fn get_class_name() -> &'static str {
        "CBaseEntity"
    }
}
impl Entity {
    define_netvar!(get_angles, ["m_angRotation"], Angles);
    define_netvar!(get_origin, ["m_vecOrigin"], Vector3);
}
impl Entity {}

//CBaseEntity{
//CBaseEntity m_ubInterpolationFrame
//CBaseEntity m_nModelIndex
//CBaseEntity m_CollisionGroup
//CBaseEntity AnimTimeMustBeFirst m_flAnimTime
//CBaseEntity m_nRenderFX
//CBaseEntity m_iTeamNum
//CBaseEntity m_Collision m_vecSpecifiedSurroundingMinsPreScaled
//CBaseEntity m_Collision m_vecSpecifiedSurroundingMaxsPreScaled
//CBaseEntity m_Collision m_vecSpecifiedSurroundingMins
//CBaseEntity m_Collision m_vecSpecifiedSurroundingMaxs
//CBaseEntity m_Collision m_vecMaxsPreScaled
//CBaseEntity m_Collision m_vecMaxs
//CBaseEntity m_Collision m_vecMinsPreScaled
//CBaseEntity m_Collision m_usSolidFlags
//CBaseEntity m_Collision m_nSolidType
//CBaseEntity m_Collision m_triggerBloat
//CBaseEntity m_Collision m_vecMins
//CBaseEntity m_Collision m_bUniformTriggerBloat
//CBaseEntity m_Collision m_nSurroundType
//CBaseEntity m_flElasticity
//CBaseEntity m_flShadowCastDistance
//CBaseEntity m_angRotation
//CBaseEntity m_bSimulatedEveryTick
//CBaseEntity predictable_id m_PredictableID
//CBaseEntity predictable_id m_bIsPlayerSimulated
//CBaseEntity m_bAnimatedEveryTick
//CBaseEntity m_iTextureFrameIndex
//CBaseEntity m_hOwnerEntity
//CBaseEntity m_fEffects
//CBaseEntity movecollide
//CBaseEntity m_nModelIndexOverrides 001
//CBaseEntity m_nModelIndexOverrides 003
//CBaseEntity m_nModelIndexOverrides 000
//CBaseEntity m_nModelIndexOverrides 002
//CBaseEntity movetype
//CBaseEntity moveparent
//CBaseEntity m_nRenderMode
//CBaseEntity m_vecOrigin
//CBaseEntity m_clrRender
//CBaseEntity m_iParentAttachment
//CBaseEntity m_flSimulationTime
//CBaseEntity m_hEffectEntity
//CBaseEntity m_bAlternateSorting
//}
