use crate::{
    call_original, cfn, get_cheat,
    math::{angles::Angles, vector3::Vector3},
    oxide::cheat::spread_reduction::{seed_prediction::State, SpreadReduction},
    sdk::entity::weapon::Weapon,
};

pub const NAME: &str = "FireBulletsServer";

pub type FireBulletsServer = cfn!(
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

    let seed = if let State::SYNCED {last_seed,..} = get_cheat!(SpreadReduction).state {
        if let Some(last_seed) = last_seed {
            last_seed
        } else {seed}
    } else {seed};
    call_original!(
        NAME,
        FireBulletsServer,
        weapon,
        player_id,
        origin,
        angle,
        weapon_id,
        mode,
        seed,
        spread,
        damage,
        critical
    );
}
