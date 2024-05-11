use crate::{define_netvar, netvars::HasNetvars};


#[derive(Debug)]
#[repr(C)]
pub struct PlayerResource{
}
impl HasNetvars for PlayerResource {
    fn get_class_name() -> &'static str {
     "CTFPlayerResource"
    }
}

impl PlayerResource {
    define_netvar!(get_damage_resource, ["m_iDamage"], [i32;102]);
}

//CTFPlayerResource m_iMaxBuffedHealth 
//CTFPlayerResource m_iMaxBuffedHealth 
//CTFPlayerResource m_iPlayerLevel 
//CTFPlayerResource m_flConnectTime 
//CTFPlayerResource m_iMaxHealth 
//CTFPlayerResource m_iPlayerClassWhenKilled 
//CTFPlayerResource m_iConnectionState 
//CTFPlayerResource m_flNextRespawnTime 
//CTFPlayerResource m_iDamageBlocked 
//CTFPlayerResource baseclass m_iHealth 
//CTFPlayerResource baseclass m_iDeaths 
//CTFPlayerResource baseclass m_iScore 
//CTFPlayerResource baseclass m_iUserID 
//CTFPlayerResource baseclass m_iTeam 
//CTFPlayerResource baseclass m_iAccountID 
//CTFPlayerResource baseclass m_iPing 
//CTFPlayerResource m_iPlayerClass 
//CTFPlayerResource m_iUpgradeRefundCredits 
//CTFPlayerResource m_iStreaks 
//CTFPlayerResource m_iHealing 
//CTFPlayerResource m_iDamageBoss 
//CTFPlayerResource m_iDamageAssist 
//CTFPlayerResource m_iTotalScore 
//CTFPlayerResource m_iCurrencyCollected 
//CTFPlayerResource m_iHealingAssist 
//CTFPlayerResource m_iActiveDominations 
//CTFPlayerResource m_iChargeLevel 
//CTFPlayerResource m_iDamage 
//CTFPlayerResource m_iBuybackCredits 
