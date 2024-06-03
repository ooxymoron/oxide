use std::mem::transmute;

use libc::c_char;

use crate::{define_netvar, netvars::HasNetvars};

pub const MAX_PLAYERS: usize = 102;

#[derive(Debug)]
#[repr(C)]
pub struct PlayerResource {}
impl HasNetvars for PlayerResource {
    fn get_class_name() -> &'static str {
        "CTFPlayerResource"
    }
}

impl PlayerResource {
    pub fn get_name(&self) -> &mut [*const c_char; MAX_PLAYERS] {
        unsafe { transmute((self as *const _ as *const u8).byte_add(0x7b8)) }
    }
}

impl PlayerResource {
    define_netvar!(get_damage_resource, ["m_iDamage"], [i32; MAX_PLAYERS]);
    define_netvar!(get_valid, ["baseclass", "m_bValid"], [bool; MAX_PLAYERS]);

    define_netvar!(
        get_max_buffed_health,
        ["m_iMaxBuffedHealth"],
        [i32; MAX_PLAYERS]
    );
    define_netvar!(get_player_level, ["m_iPlayerLevel"], [i32; MAX_PLAYERS]);
    define_netvar!(get_connect_time, ["m_flConnectTime"], [f32; MAX_PLAYERS]);
    define_netvar!(get_max_health, ["m_iMaxHealth"], [i32; MAX_PLAYERS]);
    define_netvar!(
        get_player_class_when_killed,
        ["m_iPlayerClassWhenKilled"],
        [i32; MAX_PLAYERS]
    );
    define_netvar!(
        get_connection_state,
        ["m_iConnectionState"],
        [i32; MAX_PLAYERS]
    );
    define_netvar!(
        get_next_respawn_time,
        ["m_flNextRespawnTime"],
        [f32; MAX_PLAYERS]
    );
    define_netvar!(get_damage_blocked, ["m_iDamageBlocked"], [i32; MAX_PLAYERS]);
    define_netvar!(get_health, ["baseclass", "m_iHealth"], [i32; MAX_PLAYERS]);
    define_netvar!(get_deaths, ["baseclass", "m_iDeaths"], [i32; MAX_PLAYERS]);
    define_netvar!(get_score, ["baseclass", "m_iScore"], [i32; MAX_PLAYERS]);
    define_netvar!(get_user_id, ["baseclass", "m_iUserID"], [i32; MAX_PLAYERS]);
    define_netvar!(get_team, ["baseclass", "m_iTeam"], [i32; MAX_PLAYERS]);
    define_netvar!(
        get_account_id,
        ["baseclass", "m_iAccountID"],
        [i32; MAX_PLAYERS]
    );
    define_netvar!(get_ping, ["baseclass", "m_iPing"], [i32; MAX_PLAYERS]);
    define_netvar!(get_connected, ["baseclass","m_bConnected"], [bool; MAX_PLAYERS]);
    define_netvar!(get_player_class, ["m_iPlayerClass"], [i32; MAX_PLAYERS]);
    define_netvar!(
        get_upgrade_refund_credits,
        ["m_iUpgradeRefundCredits"],
        [i32; MAX_PLAYERS]
    );
    define_netvar!(get_streaks, ["m_iStreaks"], [i32; MAX_PLAYERS]);
    define_netvar!(get_healing, ["m_iHealing"], [i32; MAX_PLAYERS]);
    define_netvar!(get_damage_boss, ["m_iDamageBoss"], [i32; MAX_PLAYERS]);
    define_netvar!(get_damage_assist, ["m_iDamageAssist"], [i32; MAX_PLAYERS]);
    define_netvar!(get_total_score, ["m_iTotalScore"], [i32; MAX_PLAYERS]);
    define_netvar!(
        get_currency_collected,
        ["m_iCurrencyCollected"],
        [i32; MAX_PLAYERS]
    );
    define_netvar!(get_healing_assist, ["m_iHealingAssist"], [i32; MAX_PLAYERS]);
    define_netvar!(
        get_active_dominations,
        ["m_iActiveDominations"],
        [i32; MAX_PLAYERS]
    );
    define_netvar!(get_charge_level, ["m_iChargeLevel"], [i32; MAX_PLAYERS]);
    define_netvar!(
        get_buyback_credits,
        ["m_iBuybackCredits"],
        [i32; MAX_PLAYERS]
    );
}
