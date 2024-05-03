use crate::{
    call_original, cfn, get_cheat,
    math::{angles::Angles, vector::Vector3},
    oxide::{cheat::spread_reduction::SpreadReduction, hook::process_user_cmds::LAST_SERVER_SEED},
    sdk::entity::weapon::Weapon,
};

pub const NAME: &str = "FireBullets";

pub type FireBullets = cfn!(
    (),
    &Weapon,
    u32,
    &Vector3,
    &Angles,
    u32,
    u32,
    i32,
    f32,
    f32,
    bool
);
pub extern "C" fn hook(
    weapon: &Weapon,
    player_id: u32,
    origin: &Vector3,
    angle: &Angles,
    weapon_id: u32,
    mode: u32,
    seed: i32,
    spread: f32,
    damage: f32,
    critical: bool,
) {
    call_original!(
        NAME,
        FireBullets,
        weapon,
        player_id,
        origin,
        angle,
        weapon_id,
        mode,
        get_cheat!(SpreadReduction).last_seed,
        spread,
        damage,
        critical
    );
}
