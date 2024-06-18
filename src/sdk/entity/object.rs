use std::mem::transmute;

use derivative::Derivative;

use crate::{define_netvar, error::OxideResult, netvars::HasNetvars, sdk::EntHandle};

use super::entity::hitbox::PlayerHitboxId;

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
    pub fn priority(&mut self) -> Option<isize> {
        if self.as_sentry().is_ok() {
            return Some(0);
        }
        Some(-1)
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
    pub fn get_hitbox_ids(&mut self) -> Vec<PlayerHitboxId> {
        let obj = self.as_object().unwrap();
        if *obj.get_mini() {
            return vec![PlayerHitboxId::Head]
        }
        match *obj.get_level() {
            ObjectLevel::BUILDING | ObjectLevel::ONE=> vec![PlayerHitboxId::Head, PlayerHitboxId::Spine1],
            ObjectLevel::TWO => vec![
                PlayerHitboxId::Head,
                PlayerHitboxId::Spine0,
                PlayerHitboxId::LeftHand,
                PlayerHitboxId::LeftLowerArm,
            ],
            ObjectLevel::THREE => vec![
                PlayerHitboxId::Head,
                PlayerHitboxId::Spine0,
                PlayerHitboxId::Spine3,
                PlayerHitboxId::LeftHip,
                PlayerHitboxId::RightUpperArm,
            ],
        }
    }
}
impl HasNetvars for Object {
    fn get_class_name() -> &'static str {
        "CBaseObject"
    }
}
impl Object {
    define_netvar!(get_level, ["m_iUpgradeLevel"], ObjectLevel);
    define_netvar!(get_mini, ["m_bMiniBuilding"], bool);
    define_netvar!(get_carried, ["m_bCarried"], bool);
    define_netvar!(get_placing, ["m_bPlacing"], bool);
    define_netvar!(get_builder, ["m_hBuilder"], EntHandle);
}

//CBaseObject{
//CBaseObject m_bCarryDeploy
//CBaseObject m_hBuiltOnEntity
//CBaseObject m_bPlasmaDisable
//CBaseObject m_iObjectMode
//CBaseObject m_iHealth
//CBaseObject m_iMaxHealth
//CBaseObject m_bCarried
//CBaseObject m_bPlacing
//CBaseObject m_iUpgradeLevel
//CBaseObject m_bDisposableBuilding
//CBaseObject m_hBuilder
//CBaseObject m_vecBuildMins
//CBaseObject m_iHighestUpgradeLevel
//CBaseObject m_iDesiredBuildRotations
//CBaseObject m_bServerOverridePlacement
//CBaseObject m_bWasMapPlaced
//CBaseObject m_iUpgradeMetalRequired
//CBaseObject m_bDisabled
//CBaseObject m_iUpgradeMetal
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 008 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 008 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 008 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 008 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 008 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 010 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 010 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 010 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 010 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 010 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 005 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 005 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 005 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 005 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 005 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 002 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 002 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 002 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 002 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 002 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 003 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 003 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 003 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 003 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 003 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 004 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 004 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 004 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 004 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 004 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 000 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 000 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 000 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 000 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 000 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay lengthproxy lengthprop15
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 001 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 001 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 001 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 001 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 001 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 013 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 013 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 013 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 013 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 013 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 014 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 014 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 014 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 014 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 014 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 011 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 011 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 011 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 011 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 011 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 007 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 007 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 007 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 007 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 007 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 006 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 006 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 006 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 006 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 006 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 009 m_nOrder
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 009 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 009 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 009 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 009 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 012 m_nSequence
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 012 m_flPrevCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 012 m_flWeight
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 012 m_flCycle
//CBaseObject baseclass baseclass baseclass overlay_vars m_AnimOverlay 012 m_nOrder
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_bSimulatedEveryTick
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_hEffectEntity
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_clrRender
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_flShadowCastDistance
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_fEffects
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_angRotation
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_bAnimatedEveryTick
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_bAlternateSorting
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_iTextureFrameIndex
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_iTeamNum
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_vecOrigin
//CBaseObject baseclass baseclass baseclass baseclass baseclass AnimTimeMustBeFirst m_flAnimTime
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_ubInterpolationFrame
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_nRenderMode
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMins
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_triggerBloat
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_usSolidFlags
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMaxs
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_nSolidType
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_bUniformTriggerBloat
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMinsPreScaled
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMaxsPreScaled
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMaxsPreScaled
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMins
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMinsPreScaled
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMaxs
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_Collision m_nSurroundType
//CBaseObject baseclass baseclass baseclass baseclass baseclass movetype
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_flElasticity
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 003
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 001
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 000
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 002
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_CollisionGroup
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_iParentAttachment
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_hOwnerEntity
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_nRenderFX
//CBaseObject baseclass baseclass baseclass baseclass baseclass movecollide
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_nModelIndex
//CBaseObject baseclass baseclass baseclass baseclass baseclass moveparent
//CBaseObject baseclass baseclass baseclass baseclass baseclass predictable_id m_bIsPlayerSimulated
//CBaseObject baseclass baseclass baseclass baseclass baseclass predictable_id m_PredictableID
//CBaseObject baseclass baseclass baseclass baseclass baseclass m_flSimulationTime
//CBaseObject baseclass baseclass baseclass baseclass m_vecForce
//CBaseObject baseclass baseclass baseclass baseclass serveranimdata m_flCycle
//CBaseObject baseclass baseclass baseclass baseclass m_nHitboxSet
//CBaseObject baseclass baseclass baseclass baseclass m_flFadeScale
//CBaseObject baseclass baseclass baseclass baseclass m_bClientSideFrameReset
//CBaseObject baseclass baseclass baseclass baseclass m_flEncodedController 003
//CBaseObject baseclass baseclass baseclass baseclass m_flEncodedController 000
//CBaseObject baseclass baseclass baseclass baseclass m_flEncodedController 002
//CBaseObject baseclass baseclass baseclass baseclass m_flEncodedController 001
//CBaseObject baseclass baseclass baseclass baseclass m_bClientSideAnimation
//CBaseObject baseclass baseclass baseclass baseclass m_nSequence
//CBaseObject baseclass baseclass baseclass baseclass m_nBody
//CBaseObject baseclass baseclass baseclass baseclass m_fadeMinDist
//CBaseObject baseclass baseclass baseclass baseclass m_flPlaybackRate
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 017
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 006
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 001
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 000
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 015
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 008
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 018
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 007
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 022
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 002
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 012
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 009
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 003
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 004
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 019
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 010
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 005
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 011
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 014
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 013
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 021
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 020
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 016
//CBaseObject baseclass baseclass baseclass baseclass m_flPoseParameter 023
//CBaseObject baseclass baseclass baseclass baseclass m_nNewSequenceParity
//CBaseObject baseclass baseclass baseclass baseclass m_nResetEventsParity
//CBaseObject baseclass baseclass baseclass baseclass m_hLightingOriginRelative
//CBaseObject baseclass baseclass baseclass baseclass m_nSkin
//CBaseObject baseclass baseclass baseclass baseclass m_hLightingOrigin
//CBaseObject baseclass baseclass baseclass baseclass m_nMuzzleFlashParity
//CBaseObject baseclass baseclass baseclass baseclass m_flModelScale
//CBaseObject baseclass baseclass baseclass baseclass m_flModelWidthScale
//CBaseObject baseclass baseclass baseclass baseclass m_nForceBone
//CBaseObject baseclass baseclass baseclass baseclass m_fadeMaxDist
//CBaseObject baseclass baseclass m_flexWeight 019
//CBaseObject baseclass baseclass m_flexWeight 052
//CBaseObject baseclass baseclass m_flexWeight 061
//CBaseObject baseclass baseclass m_flexWeight 030
//CBaseObject baseclass baseclass m_flexWeight 001
//CBaseObject baseclass baseclass m_flexWeight 000
//CBaseObject baseclass baseclass m_flexWeight 015
//CBaseObject baseclass baseclass m_flexWeight 016
//CBaseObject baseclass baseclass m_flexWeight 033
//CBaseObject baseclass baseclass m_flexWeight 043
//CBaseObject baseclass baseclass m_flexWeight 035
//CBaseObject baseclass baseclass m_flexWeight 004
//CBaseObject baseclass baseclass m_flexWeight 044
//CBaseObject baseclass baseclass m_flexWeight 053
//CBaseObject baseclass baseclass m_flexWeight 023
//CBaseObject baseclass baseclass m_flexWeight 045
//CBaseObject baseclass baseclass m_flexWeight 055
//CBaseObject baseclass baseclass m_flexWeight 062
//CBaseObject baseclass baseclass m_flexWeight 008
//CBaseObject baseclass baseclass m_flexWeight 063
//CBaseObject baseclass baseclass m_flexWeight 064
//CBaseObject baseclass baseclass m_flexWeight 039
//CBaseObject baseclass baseclass m_flexWeight 065
//CBaseObject baseclass baseclass m_flexWeight 069
//CBaseObject baseclass baseclass m_flexWeight 075
//CBaseObject baseclass baseclass m_flexWeight 080
//CBaseObject baseclass baseclass m_flexWeight 007
//CBaseObject baseclass baseclass m_flexWeight 085
//CBaseObject baseclass baseclass m_flexWeight 068
//CBaseObject baseclass baseclass m_flexWeight 086
//CBaseObject baseclass baseclass m_flexWeight 014
//CBaseObject baseclass baseclass m_flexWeight 088
//CBaseObject baseclass baseclass m_flexWeight 025
//CBaseObject baseclass baseclass m_flexWeight 067
//CBaseObject baseclass baseclass m_flexWeight 073
//CBaseObject baseclass baseclass m_flexWeight 047
//CBaseObject baseclass baseclass m_flexWeight 090
//CBaseObject baseclass baseclass m_flexWeight 089
//CBaseObject baseclass baseclass m_flexWeight 013
//CBaseObject baseclass baseclass m_flexWeight 093
//CBaseObject baseclass baseclass m_flexWeight 041
//CBaseObject baseclass baseclass m_flexWeight 017
//CBaseObject baseclass baseclass m_flexWeight 037
//CBaseObject baseclass baseclass m_flexWeight 077
//CBaseObject baseclass baseclass m_flexWeight 010
//CBaseObject baseclass baseclass m_flexWeight 074
//CBaseObject baseclass baseclass m_flexWeight 026
//CBaseObject baseclass baseclass m_flexWeight 012
//CBaseObject baseclass baseclass m_flexWeight 054
//CBaseObject baseclass baseclass m_flexWeight 076
//CBaseObject baseclass baseclass m_flexWeight 022
//CBaseObject baseclass baseclass m_flexWeight 005
//CBaseObject baseclass baseclass m_flexWeight 009
//CBaseObject baseclass baseclass m_flexWeight 042
//CBaseObject baseclass baseclass m_flexWeight 032
//CBaseObject baseclass baseclass m_flexWeight 036
//CBaseObject baseclass baseclass m_flexWeight 046
//CBaseObject baseclass baseclass m_flexWeight 049
//CBaseObject baseclass baseclass m_flexWeight 079
//CBaseObject baseclass baseclass m_flexWeight 081
//CBaseObject baseclass baseclass m_flexWeight 083
//CBaseObject baseclass baseclass m_flexWeight 084
//CBaseObject baseclass baseclass m_flexWeight 082
//CBaseObject baseclass baseclass m_flexWeight 029
//CBaseObject baseclass baseclass m_flexWeight 051
//CBaseObject baseclass baseclass m_flexWeight 034
//CBaseObject baseclass baseclass m_flexWeight 066
//CBaseObject baseclass baseclass m_flexWeight 050
//CBaseObject baseclass baseclass m_flexWeight 095
//CBaseObject baseclass baseclass m_flexWeight 087
//CBaseObject baseclass baseclass m_flexWeight 011
//CBaseObject baseclass baseclass m_flexWeight 091
//CBaseObject baseclass baseclass m_flexWeight 072
//CBaseObject baseclass baseclass m_flexWeight 031
//CBaseObject baseclass baseclass m_flexWeight 056
//CBaseObject baseclass baseclass m_flexWeight 078
//CBaseObject baseclass baseclass m_flexWeight 002
//CBaseObject baseclass baseclass m_flexWeight 028
//CBaseObject baseclass baseclass m_flexWeight 060
//CBaseObject baseclass baseclass m_flexWeight 057
//CBaseObject baseclass baseclass m_flexWeight 058
//CBaseObject baseclass baseclass m_flexWeight 040
//CBaseObject baseclass baseclass m_flexWeight 020
//CBaseObject baseclass baseclass m_flexWeight 018
//CBaseObject baseclass baseclass m_flexWeight 070
//CBaseObject baseclass baseclass m_flexWeight 071
//CBaseObject baseclass baseclass m_flexWeight 092
//CBaseObject baseclass baseclass m_flexWeight 024
//CBaseObject baseclass baseclass m_flexWeight 038
//CBaseObject baseclass baseclass m_flexWeight 048
//CBaseObject baseclass baseclass m_flexWeight 059
//CBaseObject baseclass baseclass m_flexWeight 006
//CBaseObject baseclass baseclass m_flexWeight 003
//CBaseObject baseclass baseclass m_flexWeight 021
//CBaseObject baseclass baseclass m_flexWeight 027
//CBaseObject baseclass baseclass m_flexWeight 094
//CBaseObject baseclass baseclass m_blinktoggle
//CBaseObject baseclass baseclass m_viewtarget
//CBaseObject baseclass m_hActiveWeapon
//CBaseObject baseclass m_bGlowEnabled
//CBaseObject baseclass m_hMyWeapons 043
//CBaseObject baseclass m_hMyWeapons 001
//CBaseObject baseclass m_hMyWeapons 037
//CBaseObject baseclass m_hMyWeapons 030
//CBaseObject baseclass m_hMyWeapons 027
//CBaseObject baseclass m_hMyWeapons 016
//CBaseObject baseclass m_hMyWeapons 024
//CBaseObject baseclass m_hMyWeapons 006
//CBaseObject baseclass m_hMyWeapons 017
//CBaseObject baseclass m_hMyWeapons 020
//CBaseObject baseclass m_hMyWeapons 025
//CBaseObject baseclass m_hMyWeapons 028
//CBaseObject baseclass m_hMyWeapons 005
//CBaseObject baseclass m_hMyWeapons 000
//CBaseObject baseclass m_hMyWeapons 032
//CBaseObject baseclass m_hMyWeapons 003
//CBaseObject baseclass m_hMyWeapons 038
//CBaseObject baseclass m_hMyWeapons 040
//CBaseObject baseclass m_hMyWeapons 042
//CBaseObject baseclass m_hMyWeapons 013
//CBaseObject baseclass m_hMyWeapons 007
//CBaseObject baseclass m_hMyWeapons 045
//CBaseObject baseclass m_hMyWeapons 026
//CBaseObject baseclass m_hMyWeapons 004
//CBaseObject baseclass m_hMyWeapons 015
//CBaseObject baseclass m_hMyWeapons 029
//CBaseObject baseclass m_hMyWeapons 009
//CBaseObject baseclass m_hMyWeapons 010
//CBaseObject baseclass m_hMyWeapons 019
//CBaseObject baseclass m_hMyWeapons 021
//CBaseObject baseclass m_hMyWeapons 034
//CBaseObject baseclass m_hMyWeapons 046
//CBaseObject baseclass m_hMyWeapons 044
//CBaseObject baseclass m_hMyWeapons 012
//CBaseObject baseclass m_hMyWeapons 023
//CBaseObject baseclass m_hMyWeapons 036
//CBaseObject baseclass m_hMyWeapons 018
//CBaseObject baseclass m_hMyWeapons 047
//CBaseObject baseclass m_hMyWeapons 002
//CBaseObject baseclass m_hMyWeapons 011
//CBaseObject baseclass m_hMyWeapons 022
//CBaseObject baseclass m_hMyWeapons 008
//CBaseObject baseclass m_hMyWeapons 033
//CBaseObject baseclass m_hMyWeapons 035
//CBaseObject baseclass m_hMyWeapons 039
//CBaseObject baseclass m_hMyWeapons 014
//CBaseObject baseclass m_hMyWeapons 031
//CBaseObject baseclass m_hMyWeapons 041
//CBaseObject baseclass bcc_localdata m_flNextAttack
//CBaseObject m_iObjectType
//CBaseObject m_bHasSapper
//CBaseObject m_bBuilding
//CBaseObject m_bMiniBuilding
//CBaseObject m_flPercentageConstructed
//CBaseObject m_fObjectFlags
//}
