use std::{
    intrinsics::transmute_unchecked,
    mem::{transmute, MaybeUninit},
};

use derivative::Derivative;

use crate::{
    define_netvar, define_offset,
    error::{OxideError, OxideResult},
    interface,
    math::{angles::Angles, vector3::Vector3},
    netvars::HasNetvars,
    o,
    sdk::net_channel::LatencyFlow,
    vmt_call,
};

use self::anim_state::AnimState;

use super::{
    condition::Condition, flags::Flags, interfaces::base_engine::PlayerInfo, user_cmd::UserCmd, weapon::Weapon, Entity, WaterLevel
};

pub mod anim_state;
pub mod player_class;

use player_class::PlayerClass;

pub const MAX_WEAPONS: usize = 48;

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct VMTPlayer {}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Player {
    pub vmt: *mut VMTPlayer,
}

impl Player {
    pub fn get_local() -> OxideResult<&'static mut Player> {
        let id = vmt_call!(interface!(base_engine), get_local_player);
        let Some(ent) = Entity::get_ent(id) else {
            return Err(OxideError::new("plocal is none"))
        };
        return ent.as_player();
    }
    pub fn get_byt_user_id(id: u32) -> OxideResult<&'static mut Player> {
        let id = vmt_call!(interface!(base_engine), get_player_from_user_id, id);
        let Some(ent) = Entity::get_ent(id) else {
            return Err(OxideError::new("plocal is none"))
        };
        return ent.as_player();
    }
    pub fn as_ent(&self) -> &mut Entity {
        unsafe { transmute_unchecked(self) }
    }
    pub fn can_attack(&self) -> bool {
        let weapon = vmt_call!(self.as_ent(), get_weapon);
        let net_channel = interface!(base_engine).get_net_channel().unwrap();
        let now = o!().global_vars.interval_per_tick
            * ((o!().global_vars.tick_count + 1) as f32
                + vmt_call!(net_channel, get_latency, LatencyFlow::BOTH));
        *self.get_next_attack() <= now && weapon.can_attack()
    }
    pub fn info(&self) -> OxideResult<PlayerInfo> {
        let mut info = unsafe { MaybeUninit::zeroed().assume_init() };
        let id = vmt_call!(self.as_ent().as_networkable(), get_index);
        let res = vmt_call!(interface!(base_engine), get_player_info, id, &mut info);
        if !res {
            return Err(OxideError::new("failed to get player info"));
        }
        Ok(info.into())
    }
    pub fn weapon(&self) -> &mut Weapon {
        vmt_call!(self.as_ent(), get_weapon)
    }
}

impl HasNetvars for Player {
    fn get_class_name() -> &'static str {
        "CTFPlayer"
    }
}

impl Player {
    define_netvar!(get_force_taunt_cam, ["m_nForceTauntCam"], bool);
    define_netvar!(
        get_next_attack,
        ["baseclass", "baseclass", "bcc_localdata", "m_flNextAttack"],
        f32
    );
    define_netvar!(get_condition, ["m_Shared", "m_nPlayerCond"], Condition);
    define_netvar!(
        get_punch_angle,
        ["baseclass", "localdata", "m_Local", "m_vecPunchAngle"],
        Angles
    );
    define_netvar!(
        get_tick_base,
        ["baseclass", "localdata", "m_nTickBase"],
        i32
    );
    define_netvar!(get_player_class, ["m_PlayerClass", "m_iClass"], PlayerClass);
    define_netvar!(
        get_velocity,
        ["baseclass", "localdata", "m_vecVelocity[0]"],
        Vector3
    );
    define_netvar!(
        get_water_level,
        ["baseclass", "localdata", "m_nWaterLevel"],
        WaterLevel
    );
    define_netvar!(
        get_friction,
        ["baseclass", "localdata", "m_flFriction"],
        f32
    );
}

impl Player {
    //"CPrediction::FinishCommand"
    define_offset!(get_current_command, 0x1620, &UserCmd);
    //"C_BasePlayer::PhysicsSimulate"
    define_offset!(get_flags, 0x460, Flags);
    //"spyMask"
    define_offset!(get_anim_state, 0x2348, AnimState);
}

//CTFPlayer{
//CTFPlayer m_flTauntYaw
//CTFPlayer m_iKartHealth
//CTFPlayer m_flHeadScale
//CTFPlayer m_hRagdoll
//CTFPlayer m_bUseBossHealthBar
//CTFPlayer m_hSecondaryLastWeapon
//CTFPlayer baseclass m_hObserverTarget
//CTFPlayer baseclass m_iHealth
//CTFPlayer baseclass m_iBonusChallenge
//CTFPlayer baseclass localdata m_vecViewOffset[1]
//CTFPlayer baseclass localdata m_vecVelocity[2]
//CTFPlayer baseclass localdata m_flDeathTime
//CTFPlayer baseclass localdata m_vecViewOffset[0]
//CTFPlayer baseclass localdata m_flConstraintRadius
//CTFPlayer baseclass localdata m_hConstraintEntity
//CTFPlayer baseclass localdata m_fOnTarget
//CTFPlayer baseclass localdata m_vecVelocity[1]
//CTFPlayer baseclass localdata m_vecViewOffset[2]
//CTFPlayer baseclass localdata m_vecConstraintCenter
//CTFPlayer baseclass localdata m_flConstraintSpeedFactor
//CTFPlayer baseclass localdata m_nTickBase
//CTFPlayer baseclass localdata m_nNextThinkTick
//CTFPlayer baseclass localdata m_nWaterLevel
//CTFPlayer baseclass localdata m_flLaggedMovementValue
//CTFPlayer baseclass localdata m_iAmmo 001
//CTFPlayer baseclass localdata m_iAmmo 000
//CTFPlayer baseclass localdata m_iAmmo 007
//CTFPlayer baseclass localdata m_iAmmo 016
//CTFPlayer baseclass localdata m_iAmmo 017
//CTFPlayer baseclass localdata m_iAmmo 026
//CTFPlayer baseclass localdata m_iAmmo 029
//CTFPlayer baseclass localdata m_iAmmo 023
//CTFPlayer baseclass localdata m_iAmmo 028
//CTFPlayer baseclass localdata m_iAmmo 021
//CTFPlayer baseclass localdata m_iAmmo 003
//CTFPlayer baseclass localdata m_iAmmo 009
//CTFPlayer baseclass localdata m_iAmmo 024
//CTFPlayer baseclass localdata m_iAmmo 025
//CTFPlayer baseclass localdata m_iAmmo 006
//CTFPlayer baseclass localdata m_iAmmo 019
//CTFPlayer baseclass localdata m_iAmmo 027
//CTFPlayer baseclass localdata m_iAmmo 005
//CTFPlayer baseclass localdata m_iAmmo 011
//CTFPlayer baseclass localdata m_iAmmo 013
//CTFPlayer baseclass localdata m_iAmmo 012
//CTFPlayer baseclass localdata m_iAmmo 002
//CTFPlayer baseclass localdata m_iAmmo 008
//CTFPlayer baseclass localdata m_iAmmo 018
//CTFPlayer baseclass localdata m_iAmmo 014
//CTFPlayer baseclass localdata m_iAmmo 022
//CTFPlayer baseclass localdata m_iAmmo 031
//CTFPlayer baseclass localdata m_iAmmo 015
//CTFPlayer baseclass localdata m_iAmmo 020
//CTFPlayer baseclass localdata m_iAmmo 030
//CTFPlayer baseclass localdata m_iAmmo 010
//CTFPlayer baseclass localdata m_iAmmo 004
//CTFPlayer baseclass localdata m_hLastWeapon
//CTFPlayer baseclass localdata m_vecBaseVelocity
//CTFPlayer baseclass localdata m_flFriction
//CTFPlayer baseclass localdata m_vecVelocity[0]
//CTFPlayer baseclass localdata m_Local m_skybox3d.scale
//CTFPlayer baseclass localdata m_Local m_audio.localSound[7]
//CTFPlayer baseclass localdata m_Local m_flJumpTime
//CTFPlayer baseclass localdata m_Local m_bForceLocalPlayerDraw
//CTFPlayer baseclass localdata m_Local m_audio.localBits
//CTFPlayer baseclass localdata m_Local m_audio.localSound[6]
//CTFPlayer baseclass localdata m_Local m_flDuckJumpTime
//CTFPlayer baseclass localdata m_Local m_audio.localSound[0]
//CTFPlayer baseclass localdata m_Local m_audio.entIndex
//CTFPlayer baseclass localdata m_Local m_skybox3d.fog.dirPrimary
//CTFPlayer baseclass localdata m_Local m_szScriptOverlayMaterial
//CTFPlayer baseclass localdata m_Local m_skybox3d.fog.maxdensity
//CTFPlayer baseclass localdata m_Local m_audio.soundscapeIndex
//CTFPlayer baseclass localdata m_Local m_flDucktime
//CTFPlayer baseclass localdata m_Local m_vecPunchAngle
//CTFPlayer baseclass localdata m_Local m_bDucking
//CTFPlayer baseclass localdata m_Local m_audio.localSound[3]
//CTFPlayer baseclass localdata m_Local m_bWearingSuit
//CTFPlayer baseclass localdata m_Local m_skybox3d.fog.colorSecondary
//CTFPlayer baseclass localdata m_Local m_flStepSize
//CTFPlayer baseclass localdata m_Local m_bDucked
//CTFPlayer baseclass localdata m_Local m_bPoisoned
//CTFPlayer baseclass localdata m_Local m_skybox3d.area
//CTFPlayer baseclass localdata m_Local m_PlayerFog.m_hCtrl
//CTFPlayer baseclass localdata m_Local m_skybox3d.fog.start
//CTFPlayer baseclass localdata m_Local m_flFOVRate
//CTFPlayer baseclass localdata m_Local m_skybox3d.fog.end
//CTFPlayer baseclass localdata m_Local m_skybox3d.fog.blend
//CTFPlayer baseclass localdata m_Local m_skybox3d.origin
//CTFPlayer baseclass localdata m_Local m_audio.localSound[1]
//CTFPlayer baseclass localdata m_Local m_audio.localSound[2]
//CTFPlayer baseclass localdata m_Local m_bDrawViewmodel
//CTFPlayer baseclass localdata m_Local m_bAllowAutoMovement
//CTFPlayer baseclass localdata m_Local m_skybox3d.fog.colorPrimary
//CTFPlayer baseclass localdata m_Local m_audio.localSound[4]
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 020
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 009
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 014
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 008
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 002
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 021
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 003
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 006
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 012
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 015
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 018
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 017
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 019
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 000
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 007
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 022
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 004
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 005
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 011
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 013
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 016
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 023
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 010
//CTFPlayer baseclass localdata m_Local m_chAreaPortalBits 001
//CTFPlayer baseclass localdata m_Local m_audio.localSound[5]
//CTFPlayer baseclass localdata m_Local m_vecPunchAngleVel
//CTFPlayer baseclass localdata m_Local m_chAreaBits 004
//CTFPlayer baseclass localdata m_Local m_chAreaBits 006
//CTFPlayer baseclass localdata m_Local m_chAreaBits 019
//CTFPlayer baseclass localdata m_Local m_chAreaBits 022
//CTFPlayer baseclass localdata m_Local m_chAreaBits 003
//CTFPlayer baseclass localdata m_Local m_chAreaBits 030
//CTFPlayer baseclass localdata m_Local m_chAreaBits 007
//CTFPlayer baseclass localdata m_Local m_chAreaBits 010
//CTFPlayer baseclass localdata m_Local m_chAreaBits 027
//CTFPlayer baseclass localdata m_Local m_chAreaBits 002
//CTFPlayer baseclass localdata m_Local m_chAreaBits 009
//CTFPlayer baseclass localdata m_Local m_chAreaBits 001
//CTFPlayer baseclass localdata m_Local m_chAreaBits 029
//CTFPlayer baseclass localdata m_Local m_chAreaBits 023
//CTFPlayer baseclass localdata m_Local m_chAreaBits 014
//CTFPlayer baseclass localdata m_Local m_chAreaBits 013
//CTFPlayer baseclass localdata m_Local m_chAreaBits 000
//CTFPlayer baseclass localdata m_Local m_chAreaBits 018
//CTFPlayer baseclass localdata m_Local m_chAreaBits 024
//CTFPlayer baseclass localdata m_Local m_chAreaBits 026
//CTFPlayer baseclass localdata m_Local m_chAreaBits 017
//CTFPlayer baseclass localdata m_Local m_chAreaBits 025
//CTFPlayer baseclass localdata m_Local m_chAreaBits 016
//CTFPlayer baseclass localdata m_Local m_chAreaBits 028
//CTFPlayer baseclass localdata m_Local m_chAreaBits 008
//CTFPlayer baseclass localdata m_Local m_chAreaBits 012
//CTFPlayer baseclass localdata m_Local m_chAreaBits 015
//CTFPlayer baseclass localdata m_Local m_chAreaBits 005
//CTFPlayer baseclass localdata m_Local m_chAreaBits 031
//CTFPlayer baseclass localdata m_Local m_chAreaBits 021
//CTFPlayer baseclass localdata m_Local m_chAreaBits 020
//CTFPlayer baseclass localdata m_Local m_chAreaBits 011
//CTFPlayer baseclass localdata m_Local m_skybox3d.fog.enable
//CTFPlayer baseclass localdata m_Local m_flFallVelocity
//CTFPlayer baseclass localdata m_Local m_iHideHUD
//CTFPlayer baseclass localdata m_Local m_bInDuckJump
//CTFPlayer baseclass localdata m_flConstraintWidth
//CTFPlayer baseclass localdata m_hGroundEntity
//CTFPlayer baseclass m_AttributeList m_Attributes 003 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 003 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 003 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 003 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 002 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 002 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 002 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 002 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 016 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 016 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 016 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 016 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 005 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 005 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 005 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 005 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 001 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 001 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 001 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 001 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 008 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 008 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 008 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 008 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 009 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 009 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 009 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 009 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 011 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 011 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 011 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 011 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 019 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 019 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 019 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 019 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 006 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 006 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 006 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 006 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes lengthproxy lengthprop20
//CTFPlayer baseclass m_AttributeList m_Attributes 015 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 015 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 015 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 015 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 007 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 007 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 007 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 007 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 014 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 014 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 014 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 014 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 000 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 000 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 000 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 000 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 004 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 004 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 004 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 004 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 017 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 017 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 017 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 017 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 012 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 012 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 012 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 012 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 010 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 010 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 010 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 010 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 013 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 013 m_flValue
//CTFPlayer baseclass m_AttributeList m_Attributes 013 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 013 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 018 m_iAttributeDefinitionIndex
//CTFPlayer baseclass m_AttributeList m_Attributes 018 m_iRawValue32
//CTFPlayer baseclass m_AttributeList m_Attributes 018 m_nRefundableCurrency
//CTFPlayer baseclass m_AttributeList m_Attributes 018 m_flValue
//CTFPlayer baseclass m_iDefaultFOV
//CTFPlayer baseclass m_iObserverMode
//CTFPlayer baseclass m_flMaxspeed
//CTFPlayer baseclass m_hVehicle
//CTFPlayer baseclass m_hViewModel[0]
//CTFPlayer baseclass m_szLastPlaceName
//CTFPlayer baseclass m_iFOVStart
//CTFPlayer baseclass m_hViewModel
//CTFPlayer baseclass pl deadflag
//CTFPlayer baseclass m_flFOVTime
//CTFPlayer baseclass m_fFlags
//CTFPlayer baseclass m_hZoomOwner
//CTFPlayer baseclass m_iFOV
//CTFPlayer baseclass m_iBonusProgress
//CTFPlayer baseclass m_hMyWearables 005
//CTFPlayer baseclass m_hMyWearables 001
//CTFPlayer baseclass m_hMyWearables 006
//CTFPlayer baseclass m_hMyWearables 004
//CTFPlayer baseclass m_hMyWearables 007
//CTFPlayer baseclass m_hMyWearables 002
//CTFPlayer baseclass m_hMyWearables 000
//CTFPlayer baseclass m_hMyWearables 003
//CTFPlayer baseclass m_hMyWearables lengthproxy lengthprop8
//CTFPlayer baseclass m_lifeState
//CTFPlayer baseclass m_hUseEntity
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 007 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 007 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 007 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 007 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 007 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 013 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 013 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 013 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 013 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 013 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 004 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 004 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 004 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 004 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 004 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 000 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 000 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 000 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 000 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 000 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 010 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 010 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 010 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 010 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 010 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 005 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 005 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 005 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 005 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 005 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 002 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 002 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 002 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 002 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 002 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 006 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 006 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 006 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 006 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 006 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 001 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 001 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 001 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 001 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 001 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 012 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 012 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 012 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 012 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 012 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 008 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 008 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 008 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 008 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 008 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay lengthproxy lengthprop15
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 009 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 009 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 009 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 009 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 009 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 011 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 011 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 011 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 011 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 011 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 014 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 014 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 014 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 014 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 014 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 003 m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 003 m_flWeight
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 003 m_flPrevCycle
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 003 m_nOrder
//CTFPlayer baseclass baseclass baseclass baseclass overlay_vars m_AnimOverlay 003 m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flModelWidthScale
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flEncodedController 002
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flEncodedController 001
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flEncodedController 003
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flEncodedController 000
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_hLightingOriginRelative
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_nForceBone
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_nSkin
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_fadeMinDist
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flFadeScale
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_clrRender
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_flShadowCastDistance
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_nRenderFX
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass movecollide
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass movetype
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_iTeamNum
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_flElasticity
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_ubInterpolationFrame
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_hEffectEntity
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_bSimulatedEveryTick
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_fEffects
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass moveparent
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_flSimulationTime
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_bAlternateSorting
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMaxs
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_nSurroundType
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_triggerBloat
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMinsPreScaled
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMaxsPreScaled
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMinsPreScaled
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMins
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_nSolidType
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_bUniformTriggerBloat
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMins
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMaxsPreScaled
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_usSolidFlags
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMaxs
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_bAnimatedEveryTick
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_hOwnerEntity
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_nRenderMode
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 003
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 000
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 001
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 002
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_iParentAttachment
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_vecOrigin
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass AnimTimeMustBeFirst m_flAnimTime
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_angRotation
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass predictable_id m_bIsPlayerSimulated
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass predictable_id m_PredictableID
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_nModelIndex
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_CollisionGroup
//CTFPlayer baseclass baseclass baseclass baseclass baseclass baseclass m_iTextureFrameIndex
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_nBody
//CTFPlayer baseclass baseclass baseclass baseclass baseclass serveranimdata m_flCycle
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 018
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 001
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 021
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 014
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 016
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 017
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 022
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 005
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 007
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 019
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 020
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 003
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 006
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 023
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 008
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 004
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 009
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 015
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 012
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 013
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 002
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 011
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 000
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPoseParameter 010
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_nNewSequenceParity
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_hLightingOrigin
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_bClientSideAnimation
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_nMuzzleFlashParity
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_bClientSideFrameReset
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_vecForce
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flModelScale
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_flPlaybackRate
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_fadeMaxDist
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_nResetEventsParity
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_nSequence
//CTFPlayer baseclass baseclass baseclass baseclass baseclass m_nHitboxSet
//CTFPlayer baseclass baseclass baseclass m_viewtarget
//CTFPlayer baseclass baseclass baseclass m_flexWeight 007
//CTFPlayer baseclass baseclass baseclass m_flexWeight 018
//CTFPlayer baseclass baseclass baseclass m_flexWeight 019
//CTFPlayer baseclass baseclass baseclass m_flexWeight 063
//CTFPlayer baseclass baseclass baseclass m_flexWeight 060
//CTFPlayer baseclass baseclass baseclass m_flexWeight 031
//CTFPlayer baseclass baseclass baseclass m_flexWeight 030
//CTFPlayer baseclass baseclass baseclass m_flexWeight 047
//CTFPlayer baseclass baseclass baseclass m_flexWeight 068
//CTFPlayer baseclass baseclass baseclass m_flexWeight 071
//CTFPlayer baseclass baseclass baseclass m_flexWeight 087
//CTFPlayer baseclass baseclass baseclass m_flexWeight 049
//CTFPlayer baseclass baseclass baseclass m_flexWeight 039
//CTFPlayer baseclass baseclass baseclass m_flexWeight 076
//CTFPlayer baseclass baseclass baseclass m_flexWeight 044
//CTFPlayer baseclass baseclass baseclass m_flexWeight 021
//CTFPlayer baseclass baseclass baseclass m_flexWeight 043
//CTFPlayer baseclass baseclass baseclass m_flexWeight 017
//CTFPlayer baseclass baseclass baseclass m_flexWeight 036
//CTFPlayer baseclass baseclass baseclass m_flexWeight 026
//CTFPlayer baseclass baseclass baseclass m_flexWeight 052
//CTFPlayer baseclass baseclass baseclass m_flexWeight 053
//CTFPlayer baseclass baseclass baseclass m_flexWeight 001
//CTFPlayer baseclass baseclass baseclass m_flexWeight 074
//CTFPlayer baseclass baseclass baseclass m_flexWeight 020
//CTFPlayer baseclass baseclass baseclass m_flexWeight 059
//CTFPlayer baseclass baseclass baseclass m_flexWeight 002
//CTFPlayer baseclass baseclass baseclass m_flexWeight 027
//CTFPlayer baseclass baseclass baseclass m_flexWeight 038
//CTFPlayer baseclass baseclass baseclass m_flexWeight 050
//CTFPlayer baseclass baseclass baseclass m_flexWeight 079
//CTFPlayer baseclass baseclass baseclass m_flexWeight 045
//CTFPlayer baseclass baseclass baseclass m_flexWeight 006
//CTFPlayer baseclass baseclass baseclass m_flexWeight 084
//CTFPlayer baseclass baseclass baseclass m_flexWeight 016
//CTFPlayer baseclass baseclass baseclass m_flexWeight 042
//CTFPlayer baseclass baseclass baseclass m_flexWeight 075
//CTFPlayer baseclass baseclass baseclass m_flexWeight 091
//CTFPlayer baseclass baseclass baseclass m_flexWeight 081
//CTFPlayer baseclass baseclass baseclass m_flexWeight 004
//CTFPlayer baseclass baseclass baseclass m_flexWeight 005
//CTFPlayer baseclass baseclass baseclass m_flexWeight 011
//CTFPlayer baseclass baseclass baseclass m_flexWeight 012
//CTFPlayer baseclass baseclass baseclass m_flexWeight 065
//CTFPlayer baseclass baseclass baseclass m_flexWeight 077
//CTFPlayer baseclass baseclass baseclass m_flexWeight 078
//CTFPlayer baseclass baseclass baseclass m_flexWeight 088
//CTFPlayer baseclass baseclass baseclass m_flexWeight 067
//CTFPlayer baseclass baseclass baseclass m_flexWeight 025
//CTFPlayer baseclass baseclass baseclass m_flexWeight 023
//CTFPlayer baseclass baseclass baseclass m_flexWeight 037
//CTFPlayer baseclass baseclass baseclass m_flexWeight 040
//CTFPlayer baseclass baseclass baseclass m_flexWeight 094
//CTFPlayer baseclass baseclass baseclass m_flexWeight 070
//CTFPlayer baseclass baseclass baseclass m_flexWeight 041
//CTFPlayer baseclass baseclass baseclass m_flexWeight 072
//CTFPlayer baseclass baseclass baseclass m_flexWeight 083
//CTFPlayer baseclass baseclass baseclass m_flexWeight 003
//CTFPlayer baseclass baseclass baseclass m_flexWeight 009
//CTFPlayer baseclass baseclass baseclass m_flexWeight 022
//CTFPlayer baseclass baseclass baseclass m_flexWeight 046
//CTFPlayer baseclass baseclass baseclass m_flexWeight 055
//CTFPlayer baseclass baseclass baseclass m_flexWeight 064
//CTFPlayer baseclass baseclass baseclass m_flexWeight 057
//CTFPlayer baseclass baseclass baseclass m_flexWeight 069
//CTFPlayer baseclass baseclass baseclass m_flexWeight 093
//CTFPlayer baseclass baseclass baseclass m_flexWeight 051
//CTFPlayer baseclass baseclass baseclass m_flexWeight 048
//CTFPlayer baseclass baseclass baseclass m_flexWeight 061
//CTFPlayer baseclass baseclass baseclass m_flexWeight 066
//CTFPlayer baseclass baseclass baseclass m_flexWeight 073
//CTFPlayer baseclass baseclass baseclass m_flexWeight 082
//CTFPlayer baseclass baseclass baseclass m_flexWeight 054
//CTFPlayer baseclass baseclass baseclass m_flexWeight 032
//CTFPlayer baseclass baseclass baseclass m_flexWeight 056
//CTFPlayer baseclass baseclass baseclass m_flexWeight 024
//CTFPlayer baseclass baseclass baseclass m_flexWeight 000
//CTFPlayer baseclass baseclass baseclass m_flexWeight 010
//CTFPlayer baseclass baseclass baseclass m_flexWeight 013
//CTFPlayer baseclass baseclass baseclass m_flexWeight 014
//CTFPlayer baseclass baseclass baseclass m_flexWeight 015
//CTFPlayer baseclass baseclass baseclass m_flexWeight 029
//CTFPlayer baseclass baseclass baseclass m_flexWeight 080
//CTFPlayer baseclass baseclass baseclass m_flexWeight 034
//CTFPlayer baseclass baseclass baseclass m_flexWeight 085
//CTFPlayer baseclass baseclass baseclass m_flexWeight 058
//CTFPlayer baseclass baseclass baseclass m_flexWeight 086
//CTFPlayer baseclass baseclass baseclass m_flexWeight 090
//CTFPlayer baseclass baseclass baseclass m_flexWeight 092
//CTFPlayer baseclass baseclass baseclass m_flexWeight 089
//CTFPlayer baseclass baseclass baseclass m_flexWeight 035
//CTFPlayer baseclass baseclass baseclass m_flexWeight 062
//CTFPlayer baseclass baseclass baseclass m_flexWeight 033
//CTFPlayer baseclass baseclass baseclass m_flexWeight 095
//CTFPlayer baseclass baseclass baseclass m_flexWeight 028
//CTFPlayer baseclass baseclass baseclass m_flexWeight 008
//CTFPlayer baseclass baseclass baseclass m_blinktoggle
//CTFPlayer baseclass baseclass m_hMyWeapons 042
//CTFPlayer baseclass baseclass m_hMyWeapons 010
//CTFPlayer baseclass baseclass m_hMyWeapons 008
//CTFPlayer baseclass baseclass m_hMyWeapons 004
//CTFPlayer baseclass baseclass m_hMyWeapons 017
//CTFPlayer baseclass baseclass m_hMyWeapons 032
//CTFPlayer baseclass baseclass m_hMyWeapons 035
//CTFPlayer baseclass baseclass m_hMyWeapons 046
//CTFPlayer baseclass baseclass m_hMyWeapons 006
//CTFPlayer baseclass baseclass m_hMyWeapons 003
//CTFPlayer baseclass baseclass m_hMyWeapons 029
//CTFPlayer baseclass baseclass m_hMyWeapons 024
//CTFPlayer baseclass baseclass m_hMyWeapons 031
//CTFPlayer baseclass baseclass m_hMyWeapons 023
//CTFPlayer baseclass baseclass m_hMyWeapons 022
//CTFPlayer baseclass baseclass m_hMyWeapons 005
//CTFPlayer baseclass baseclass m_hMyWeapons 015
//CTFPlayer baseclass baseclass m_hMyWeapons 020
//CTFPlayer baseclass baseclass m_hMyWeapons 021
//CTFPlayer baseclass baseclass m_hMyWeapons 036
//CTFPlayer baseclass baseclass m_hMyWeapons 012
//CTFPlayer baseclass baseclass m_hMyWeapons 038
//CTFPlayer baseclass baseclass m_hMyWeapons 039
//CTFPlayer baseclass baseclass m_hMyWeapons 043
//CTFPlayer baseclass baseclass m_hMyWeapons 002
//CTFPlayer baseclass baseclass m_hMyWeapons 045
//CTFPlayer baseclass baseclass m_hMyWeapons 011
//CTFPlayer baseclass baseclass m_hMyWeapons 033
//CTFPlayer baseclass baseclass m_hMyWeapons 000
//CTFPlayer baseclass baseclass m_hMyWeapons 047
//CTFPlayer baseclass baseclass m_hMyWeapons 034
//CTFPlayer baseclass baseclass m_hMyWeapons 007
//CTFPlayer baseclass baseclass m_hMyWeapons 037
//CTFPlayer baseclass baseclass m_hMyWeapons 030
//CTFPlayer baseclass baseclass m_hMyWeapons 009
//CTFPlayer baseclass baseclass m_hMyWeapons 016
//CTFPlayer baseclass baseclass m_hMyWeapons 014
//CTFPlayer baseclass baseclass m_hMyWeapons 013
//CTFPlayer baseclass baseclass m_hMyWeapons 028
//CTFPlayer baseclass baseclass m_hMyWeapons 040
//CTFPlayer baseclass baseclass m_hMyWeapons 041
//CTFPlayer baseclass baseclass m_hMyWeapons 044
//CTFPlayer baseclass baseclass m_hMyWeapons 026
//CTFPlayer baseclass baseclass m_hMyWeapons 001
//CTFPlayer baseclass baseclass m_hMyWeapons 027
//CTFPlayer baseclass baseclass m_hMyWeapons 018
//CTFPlayer baseclass baseclass m_hMyWeapons 019
//CTFPlayer baseclass baseclass m_hMyWeapons 025
//CTFPlayer baseclass baseclass m_hActiveWeapon
//CTFPlayer baseclass baseclass m_bGlowEnabled
//CTFPlayer baseclass baseclass bcc_localdata m_flNextAttack
//CTFPlayer tflocaldata m_angEyeAngles[1]
//CTFPlayer tflocaldata m_vecOrigin
//CTFPlayer tflocaldata m_bIsCoaching
//CTFPlayer tflocaldata m_angEyeAngles[0]
//CTFPlayer tflocaldata m_nCurrency
//CTFPlayer tflocaldata m_bMatchSafeToLeave
//CTFPlayer tflocaldata m_vecOrigin[2]
//CTFPlayer tflocaldata "player_object_array"
//CTFPlayer tflocaldata m_nExperienceLevel
//CTFPlayer tflocaldata m_nExperienceLevelProgress
//CTFPlayer tflocaldata m_hStudent
//CTFPlayer tflocaldata m_hCoach
//CTFPlayer tflocaldata player_object_array_element
//CTFPlayer TFSendHealersDataTable m_nActiveWpnClip
//CTFPlayer m_PlayerClass m_bCustomModelRotationSet
//CTFPlayer m_PlayerClass m_iClassModelParity
//CTFPlayer m_PlayerClass m_bUseClassAnimations
//CTFPlayer m_PlayerClass m_vecCustomModelOffset
//CTFPlayer m_PlayerClass m_angCustomModelRotation
//CTFPlayer m_PlayerClass m_bCustomModelRotates
//CTFPlayer m_PlayerClass m_iszCustomModel
//CTFPlayer m_PlayerClass m_iszClassIcon
//CTFPlayer m_PlayerClass m_iClass
//CTFPlayer m_PlayerClass m_bCustomModelVisibleToSelf
//CTFPlayer m_bIsMiniBoss
//CTFPlayer m_bIsABot
//CTFPlayer m_nForcedSkin
//CTFPlayer m_AttributeManager m_hOuter
//CTFPlayer m_AttributeManager m_ProviderType
//CTFPlayer m_AttributeManager m_iReapplyProvisionParity
//CTFPlayer m_Shared m_iMovementStunParity
//CTFPlayer m_Shared m_nDisguiseSkinOverride
//CTFPlayer m_Shared m_nAirDucked
//CTFPlayer m_Shared m_hCarriedObject
//CTFPlayer m_Shared tfsharedlocaldata m_flStealthNoAttackExpire
//CTFPlayer m_Shared tfsharedlocaldata m_bLastDisguisedAsOwnTeam
//CTFPlayer m_Shared tfsharedlocaldata m_flNextRageEarnTime
//CTFPlayer m_Shared tfsharedlocaldata m_nDesiredDisguiseClass
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iInvulns
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iBonusPoints
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iDefenses
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iSuicides
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iPoints
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iCaptures
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iDominations
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iBuildingsBuilt
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iBuildingsDestroyed
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iCrits
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iResupplyPoints
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iDamageDone
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iKillAssists
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iRevenge
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iHeadshots
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iBackstabs
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iHealPoints
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iTeleports
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iKills
//CTFPlayer m_Shared tfsharedlocaldata m_ScoreData m_iDeaths
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iDefenses
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iPoints
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iInvulns
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iTeleports
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iCaptures
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iKills
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iRevenge
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iBuildingsBuilt
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iDominations
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iDeaths
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iCrits
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iHealPoints
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iHeadshots
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iKillAssists
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iResupplyPoints
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iBackstabs
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iBonusPoints
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iSuicides
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iBuildingsDestroyed
//CTFPlayer m_Shared tfsharedlocaldata m_RoundScoreData m_iDamageDone
//CTFPlayer m_Shared tfsharedlocaldata m_bInUpgradeZone
//CTFPlayer m_Shared tfsharedlocaldata m_bRageDraining
//CTFPlayer m_Shared tfsharedlocaldata m_flItemChargeMeter 010
//CTFPlayer m_Shared tfsharedlocaldata m_flItemChargeMeter 000
//CTFPlayer m_Shared tfsharedlocaldata m_flItemChargeMeter 001
//CTFPlayer m_Shared tfsharedlocaldata m_flItemChargeMeter 003
//CTFPlayer m_Shared tfsharedlocaldata m_flItemChargeMeter 007
//CTFPlayer m_Shared tfsharedlocaldata m_flItemChargeMeter 005
//CTFPlayer m_Shared tfsharedlocaldata m_flItemChargeMeter 002
//CTFPlayer m_Shared tfsharedlocaldata m_flItemChargeMeter 004
//CTFPlayer m_Shared tfsharedlocaldata m_flItemChargeMeter 006
//CTFPlayer m_Shared tfsharedlocaldata m_flItemChargeMeter 008
//CTFPlayer m_Shared tfsharedlocaldata m_flItemChargeMeter 009
//CTFPlayer m_Shared tfsharedlocaldata m_bPlayerDominated
//CTFPlayer m_Shared tfsharedlocaldata m_nDesiredDisguiseTeam
//CTFPlayer m_Shared tfsharedlocaldata m_flStealthNextChangeTime
//CTFPlayer m_Shared tfsharedlocaldata m_bPlayerDominatingMe
//CTFPlayer m_Shared tfsharedlocaldata m_flRageMeter
//CTFPlayer m_Shared m_iDisguiseHealth
//CTFPlayer m_Shared m_hPasstimePassTarget
//CTFPlayer m_Shared m_nTeamTeleporterUsed
//CTFPlayer m_Shared m_flNextNoiseMakerTime
//CTFPlayer m_Shared m_iTauntConcept
//CTFPlayer m_Shared m_nDisguiseClass
//CTFPlayer m_Shared m_flInvisChangeCompleteTime
//CTFPlayer m_Shared m_iTauntIndex
//CTFPlayer m_Shared m_iCritMult
//CTFPlayer m_Shared m_iNextMeleeCrit
//CTFPlayer m_Shared m_flChargeMeter
//CTFPlayer m_Shared m_bCarryingObject
//CTFPlayer m_Shared m_flCloakMeter
//CTFPlayer m_Shared m_bLoadoutUnavailable
//CTFPlayer m_Shared m_iDesiredPlayerClass
//CTFPlayer m_Shared m_flSpyTranqBuffDuration
//CTFPlayer m_Shared m_hDisguiseTarget
//CTFPlayer m_Shared m_nPlayerCondEx4
//CTFPlayer m_Shared m_bJumping
//CTFPlayer m_Shared m_iDisguiseBody
//CTFPlayer m_Shared m_iWeaponKnockbackID
//CTFPlayer m_Shared m_ConditionList _condition_bits
//CTFPlayer m_Shared m_bShieldEquipped
//CTFPlayer m_Shared m_bHasPasstimeBall
//CTFPlayer m_Shared m_iStunIndex
//CTFPlayer m_Shared m_iMovementStunAmount
//CTFPlayer m_Shared m_unTauntSourceItemID_High
//CTFPlayer m_Shared m_hDisguiseWeapon
//CTFPlayer m_Shared m_hSwitchTo
//CTFPlayer m_Shared m_nPlayerCondEx2
//CTFPlayer m_Shared m_bFeignDeathReady
//CTFPlayer m_Shared m_flEnergyDrinkMeter
//CTFPlayer m_Shared m_flHypeMeter
//CTFPlayer m_Shared m_nStreaks 001
//CTFPlayer m_Shared m_nStreaks 002
//CTFPlayer m_Shared m_nStreaks 000
//CTFPlayer m_Shared m_nStreaks 003
//CTFPlayer m_Shared m_nPlayerCond
//CTFPlayer m_Shared m_iSpawnRoomTouchCount
//CTFPlayer m_Shared m_askForBallTime
//CTFPlayer m_Shared m_bIsTargetedForPasstimePass
//CTFPlayer m_Shared m_bKingRuneBuffActive
//CTFPlayer m_Shared m_iDecapitations
//CTFPlayer m_Shared m_nArenaNumChanges
//CTFPlayer m_Shared m_hStunner
//CTFPlayer m_Shared m_nMaskClass
//CTFPlayer m_Shared m_iStunFlags
//CTFPlayer m_Shared m_flMovementStunTime
//CTFPlayer m_Shared m_iKillCountSinceLastDeploy
//CTFPlayer m_Shared m_bArenaFirstBloodBoost
//CTFPlayer m_Shared m_bParachuteEquipped
//CTFPlayer m_Shared m_nPlayerCondEx3
//CTFPlayer m_Shared m_ConditionData 072 m_pProvider
//CTFPlayer m_Shared m_ConditionData 057 m_pProvider
//CTFPlayer m_Shared m_ConditionData 044 m_pProvider
//CTFPlayer m_Shared m_ConditionData 104 m_pProvider
//CTFPlayer m_Shared m_ConditionData 053 m_pProvider
//CTFPlayer m_Shared m_ConditionData 065 m_pProvider
//CTFPlayer m_Shared m_ConditionData 076 m_pProvider
//CTFPlayer m_Shared m_ConditionData 078 m_pProvider
//CTFPlayer m_Shared m_ConditionData 100 m_pProvider
//CTFPlayer m_Shared m_ConditionData 026 m_pProvider
//CTFPlayer m_Shared m_ConditionData 106 m_pProvider
//CTFPlayer m_Shared m_ConditionData 063 m_pProvider
//CTFPlayer m_Shared m_ConditionData 020 m_pProvider
//CTFPlayer m_Shared m_ConditionData 010 m_pProvider
//CTFPlayer m_Shared m_ConditionData 071 m_pProvider
//CTFPlayer m_Shared m_ConditionData 090 m_pProvider
//CTFPlayer m_Shared m_ConditionData 004 m_pProvider
//CTFPlayer m_Shared m_ConditionData 021 m_pProvider
//CTFPlayer m_Shared m_ConditionData 115 m_pProvider
//CTFPlayer m_Shared m_ConditionData 130 m_pProvider
//CTFPlayer m_Shared m_ConditionData 001 m_pProvider
//CTFPlayer m_Shared m_ConditionData 031 m_pProvider
//CTFPlayer m_Shared m_ConditionData 037 m_pProvider
//CTFPlayer m_Shared m_ConditionData 036 m_pProvider
//CTFPlayer m_Shared m_ConditionData 116 m_pProvider
//CTFPlayer m_Shared m_ConditionData 056 m_pProvider
//CTFPlayer m_Shared m_ConditionData 023 m_pProvider
//CTFPlayer m_Shared m_ConditionData 043 m_pProvider
//CTFPlayer m_Shared m_ConditionData 015 m_pProvider
//CTFPlayer m_Shared m_ConditionData 024 m_pProvider
//CTFPlayer m_Shared m_ConditionData 086 m_pProvider
//CTFPlayer m_Shared m_ConditionData 094 m_pProvider
//CTFPlayer m_Shared m_ConditionData 109 m_pProvider
//CTFPlayer m_Shared m_ConditionData 108 m_pProvider
//CTFPlayer m_Shared m_ConditionData 034 m_pProvider
//CTFPlayer m_Shared m_ConditionData 008 m_pProvider
//CTFPlayer m_Shared m_ConditionData 114 m_pProvider
//CTFPlayer m_Shared m_ConditionData 006 m_pProvider
//CTFPlayer m_Shared m_ConditionData 127 m_pProvider
//CTFPlayer m_Shared m_ConditionData 093 m_pProvider
//CTFPlayer m_Shared m_ConditionData 041 m_pProvider
//CTFPlayer m_Shared m_ConditionData 073 m_pProvider
//CTFPlayer m_Shared m_ConditionData 062 m_pProvider
//CTFPlayer m_Shared m_ConditionData 013 m_pProvider
//CTFPlayer m_Shared m_ConditionData 066 m_pProvider
//CTFPlayer m_Shared m_ConditionData 117 m_pProvider
//CTFPlayer m_Shared m_ConditionData 102 m_pProvider
//CTFPlayer m_Shared m_ConditionData 000 m_pProvider
//CTFPlayer m_Shared m_ConditionData 088 m_pProvider
//CTFPlayer m_Shared m_ConditionData 025 m_pProvider
//CTFPlayer m_Shared m_ConditionData 012 m_pProvider
//CTFPlayer m_Shared m_ConditionData 091 m_pProvider
//CTFPlayer m_Shared m_ConditionData 007 m_pProvider
//CTFPlayer m_Shared m_ConditionData 052 m_pProvider
//CTFPlayer m_Shared m_ConditionData 080 m_pProvider
//CTFPlayer m_Shared m_ConditionData 022 m_pProvider
//CTFPlayer m_Shared m_ConditionData lengthproxy lengthprop131
//CTFPlayer m_Shared m_ConditionData 068 m_pProvider
//CTFPlayer m_Shared m_ConditionData 075 m_pProvider
//CTFPlayer m_Shared m_ConditionData 032 m_pProvider
//CTFPlayer m_Shared m_ConditionData 087 m_pProvider
//CTFPlayer m_Shared m_ConditionData 089 m_pProvider
//CTFPlayer m_Shared m_ConditionData 128 m_pProvider
//CTFPlayer m_Shared m_ConditionData 005 m_pProvider
//CTFPlayer m_Shared m_ConditionData 083 m_pProvider
//CTFPlayer m_Shared m_ConditionData 050 m_pProvider
//CTFPlayer m_Shared m_ConditionData 119 m_pProvider
//CTFPlayer m_Shared m_ConditionData 059 m_pProvider
//CTFPlayer m_Shared m_ConditionData 092 m_pProvider
//CTFPlayer m_Shared m_ConditionData 118 m_pProvider
//CTFPlayer m_Shared m_ConditionData 120 m_pProvider
//CTFPlayer m_Shared m_ConditionData 003 m_pProvider
//CTFPlayer m_Shared m_ConditionData 122 m_pProvider
//CTFPlayer m_Shared m_ConditionData 055 m_pProvider
//CTFPlayer m_Shared m_ConditionData 061 m_pProvider
//CTFPlayer m_Shared m_ConditionData 125 m_pProvider
//CTFPlayer m_Shared m_ConditionData 045 m_pProvider
//CTFPlayer m_Shared m_ConditionData 126 m_pProvider
//CTFPlayer m_Shared m_ConditionData 017 m_pProvider
//CTFPlayer m_Shared m_ConditionData 081 m_pProvider
//CTFPlayer m_Shared m_ConditionData 009 m_pProvider
//CTFPlayer m_Shared m_ConditionData 054 m_pProvider
//CTFPlayer m_Shared m_ConditionData 074 m_pProvider
//CTFPlayer m_Shared m_ConditionData 085 m_pProvider
//CTFPlayer m_Shared m_ConditionData 028 m_pProvider
//CTFPlayer m_Shared m_ConditionData 029 m_pProvider
//CTFPlayer m_Shared m_ConditionData 011 m_pProvider
//CTFPlayer m_Shared m_ConditionData 018 m_pProvider
//CTFPlayer m_Shared m_ConditionData 016 m_pProvider
//CTFPlayer m_Shared m_ConditionData 069 m_pProvider
//CTFPlayer m_Shared m_ConditionData 070 m_pProvider
//CTFPlayer m_Shared m_ConditionData 113 m_pProvider
//CTFPlayer m_Shared m_ConditionData 048 m_pProvider
//CTFPlayer m_Shared m_ConditionData 124 m_pProvider
//CTFPlayer m_Shared m_ConditionData 042 m_pProvider
//CTFPlayer m_Shared m_ConditionData 033 m_pProvider
//CTFPlayer m_Shared m_ConditionData 040 m_pProvider
//CTFPlayer m_Shared m_ConditionData 014 m_pProvider
//CTFPlayer m_Shared m_ConditionData 064 m_pProvider
//CTFPlayer m_Shared m_ConditionData 038 m_pProvider
//CTFPlayer m_Shared m_ConditionData 079 m_pProvider
//CTFPlayer m_Shared m_ConditionData 123 m_pProvider
//CTFPlayer m_Shared m_ConditionData 099 m_pProvider
//CTFPlayer m_Shared m_ConditionData 121 m_pProvider
//CTFPlayer m_Shared m_ConditionData 110 m_pProvider
//CTFPlayer m_Shared m_ConditionData 051 m_pProvider
//CTFPlayer m_Shared m_ConditionData 112 m_pProvider
//CTFPlayer m_Shared m_ConditionData 058 m_pProvider
//CTFPlayer m_Shared m_ConditionData 067 m_pProvider
//CTFPlayer m_Shared m_ConditionData 097 m_pProvider
//CTFPlayer m_Shared m_ConditionData 084 m_pProvider
//CTFPlayer m_Shared m_ConditionData 096 m_pProvider
//CTFPlayer m_Shared m_ConditionData 095 m_pProvider
//CTFPlayer m_Shared m_ConditionData 027 m_pProvider
//CTFPlayer m_Shared m_ConditionData 082 m_pProvider
//CTFPlayer m_Shared m_ConditionData 111 m_pProvider
//CTFPlayer m_Shared m_ConditionData 077 m_pProvider
//CTFPlayer m_Shared m_ConditionData 103 m_pProvider
//CTFPlayer m_Shared m_ConditionData 049 m_pProvider
//CTFPlayer m_Shared m_ConditionData 107 m_pProvider
//CTFPlayer m_Shared m_ConditionData 060 m_pProvider
//CTFPlayer m_Shared m_ConditionData 098 m_pProvider
//CTFPlayer m_Shared m_ConditionData 105 m_pProvider
//CTFPlayer m_Shared m_ConditionData 129 m_pProvider
//CTFPlayer m_Shared m_ConditionData 002 m_pProvider
//CTFPlayer m_Shared m_ConditionData 019 m_pProvider
//CTFPlayer m_Shared m_ConditionData 035 m_pProvider
//CTFPlayer m_Shared m_ConditionData 039 m_pProvider
//CTFPlayer m_Shared m_ConditionData 101 m_pProvider
//CTFPlayer m_Shared m_ConditionData 047 m_pProvider
//CTFPlayer m_Shared m_ConditionData 046 m_pProvider
//CTFPlayer m_Shared m_ConditionData 030 m_pProvider
//CTFPlayer m_Shared m_nNumHealers
//CTFPlayer m_Shared m_unTauntSourceItemID_Low
//CTFPlayer m_Shared m_flRuneCharge
//CTFPlayer m_Shared m_flFirstPrimaryAttack
//CTFPlayer m_Shared m_nDisguiseTeam
//CTFPlayer m_Shared m_iItemFindBonus
//CTFPlayer m_Shared m_flHolsterAnimTime
//CTFPlayer m_Shared m_nHalloweenBombHeadStage
//CTFPlayer m_Shared m_iRevengeCrits
//CTFPlayer m_Shared m_iAirDash
//CTFPlayer m_Shared m_nPlayerState
//CTFPlayer m_Shared m_nPlayerCondEx
//CTFPlayer m_Shared m_flDuckTimer
//CTFPlayer m_bAllowMoveDuringTaunt
//CTFPlayer m_flTorsoScale
//CTFPlayer m_bGlowEnabled
//CTFPlayer m_flInspectTime
//CTFPlayer m_nWaterLevel
//CTFPlayer m_iSpawnCounter
//CTFPlayer m_bSaveMeParity
//CTFPlayer m_bForcedSkin
//CTFPlayer m_hItem
//CTFPlayer m_bUsingActionSlot
//CTFPlayer m_flHelpmeButtonPressTime
//CTFPlayer m_bRegenerating
//CTFPlayer m_bInPowerPlay
//CTFPlayer m_bIsReadyToHighFive
//CTFPlayer "m_flLastDamageTime"
//CTFPlayer m_iCampaignMedals
//CTFPlayer m_iTauntItemDefIndex
//CTFPlayer m_flKartNextAvailableBoost
//CTFPlayer m_hHighFivePartner
//CTFPlayer m_nActiveTauntSlot
//CTFPlayer tfnonlocaldata m_angEyeAngles[0]
//CTFPlayer tfnonlocaldata m_angEyeAngles[1]
//CTFPlayer tfnonlocaldata m_vecOrigin
//CTFPlayer tfnonlocaldata m_vecOrigin[2]
//CTFPlayer m_flCurrentTauntMoveSpeed
//CTFPlayer m_hGrapplingHookTarget
//CTFPlayer m_flMvMLastDamageTime
//CTFPlayer m_iKartState
//CTFPlayer m_nForceTauntCam
//CTFPlayer m_flVehicleReverseTime
//CTFPlayer m_bUsingVRHeadset
//CTFPlayer m_nBotSkill
//CTFPlayer m_iPlayerSkinOverride
//CTFPlayer m_bViewingCYOAPDA
//CTFPlayer m_bArenaSpectator
//CTFPlayer m_flHandScale
//}
