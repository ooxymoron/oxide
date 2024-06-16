use derivative::Derivative;

use crate::{define_netvar, netvars::HasNetvars};

#[derive(Debug, Clone, Copy)]
pub enum PipeType {
    REGULAR = 0,
    RemoteDetonate,
    RemoteDetonatePractice,
    CANNONBALL,
}
impl PipeType {
    pub fn to_str(&self) -> &str {
        match self {
            PipeType::REGULAR => "pipe",
            PipeType::RemoteDetonate => "sticky",
            PipeType::RemoteDetonatePractice => "practice sticky",
            PipeType::CANNONBALL => "cannonball",
        }
    }
}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct PipeBomb {}

impl HasNetvars for PipeBomb {
    fn get_class_name() -> &'static str {
        "CTFGrenadePipebombProjectile"
    }
}
impl PipeBomb {
    define_netvar!(get_type, ["m_iType"], PipeType);
    define_netvar!(get_touched, ["m_bTouched"], bool);
}

//CTFGrenadePipebombProjectile{
//CTFGrenadePipebombProjectile baseclass m_vecOrigin
//CTFGrenadePipebombProjectile baseclass m_bCritical
//CTFGrenadePipebombProjectile baseclass m_vInitialVelocity
//CTFGrenadePipebombProjectile baseclass m_iDeflected
//CTFGrenadePipebombProjectile baseclass baseclass m_vecVelocity
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_nSkin
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_fadeMaxDist
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_nResetEventsParity
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flEncodedController 001
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flEncodedController 003
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flEncodedController 000
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flEncodedController 002
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass serveranimdata m_flCycle
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_vecForce
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_nNewSequenceParity
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_nForceBone
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 010
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 007
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 011
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 005
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 012
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 013
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 001
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 009
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 015
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 017
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 018
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 022
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 023
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 014
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 019
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 004
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 016
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 020
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 003
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 006
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 021
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 000
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 008
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPoseParameter 002
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flPlaybackRate
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_bClientSideAnimation
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_hLightingOrigin
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_nMuzzleFlashParity
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_nBody
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_nRenderMode
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_CollisionGroup
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_hOwnerEntity
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_clrRender
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_flSimulationTime
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_ubInterpolationFrame
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_nRenderFX
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_fEffects
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass predictable_id m_PredictableID
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass predictable_id m_bIsPlayerSimulated
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass moveparent
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_hEffectEntity
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_iParentAttachment
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_nSolidType
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_triggerBloat
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMins
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMinsPreScaled
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMaxsPreScaled
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMaxs
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_usSolidFlags
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMaxs
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_vecMinsPreScaled
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_nSurroundType
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMaxsPreScaled
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMins
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_Collision m_bUniformTriggerBloat
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_angRotation
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass movetype
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_nModelIndex
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_flElasticity
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_bAnimatedEveryTick
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_vecOrigin
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_bAlternateSorting
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass AnimTimeMustBeFirst m_flAnimTime
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 000
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 001
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 002
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_nModelIndexOverrides 003
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_iTeamNum
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass movecollide
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_flShadowCastDistance
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_bSimulatedEveryTick
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass baseclass m_iTextureFrameIndex
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_bClientSideFrameReset
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flModelScale
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flFadeScale
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_flModelWidthScale
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_nSequence
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_fadeMinDist
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_nHitboxSet
//CTFGrenadePipebombProjectile baseclass baseclass baseclass baseclass m_hLightingOriginRelative
//CTFGrenadePipebombProjectile baseclass baseclass baseclass m_hOriginalLauncher
//CTFGrenadePipebombProjectile baseclass baseclass m_fFlags
//CTFGrenadePipebombProjectile baseclass baseclass m_hThrower
//CTFGrenadePipebombProjectile baseclass baseclass m_bIsLive
//CTFGrenadePipebombProjectile baseclass baseclass m_flDamage
//CTFGrenadePipebombProjectile baseclass baseclass m_DmgRadius
//CTFGrenadePipebombProjectile baseclass m_hDeflectOwner
//CTFGrenadePipebombProjectile baseclass m_angRotation
//CTFGrenadePipebombProjectile m_bTouched
//CTFGrenadePipebombProjectile m_iType
//CTFGrenadePipebombProjectile m_hLauncher
//CTFGrenadePipebombProjectile m_bDefensiveBomb
//}
