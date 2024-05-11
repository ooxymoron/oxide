use std::borrow::BorrowMut;

use crate::{
    call_original, cfn, get_cheat, interface,
    math::{angles::Angles, vector3::Vector3},
    oxide::cheat::{
        spread_reduction::{seed_prediction::State, SpreadReduction},
        visual::Visuals,
    },
    sdk::entity::weapon::Weapon,
    vmt_call,
};

pub const NAME: &str = "FireBullets";

pub type FireBullets = cfn!(
    (),
    &Weapon,
    i32,
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
    player_id: i32,
    origin: &Vector3,
    angles: &Angles,
    weapon_id: u32,
    mode: u32,
    mut seed: i32,
    spread: f32,
    damage: f32,
    critical: bool,
) {
    if player_id == vmt_call!(interface!(base_engine), get_local_player) {
        seed = if let State::SYNCED { last_seed, .. } = get_cheat!(SpreadReduction).state.borrow_mut() {
            let seed = if let Some(last_seed) = last_seed {
                *last_seed
            } else {
                seed
            };
            *last_seed = None;

            seed
        } else {
            seed
        };
        get_cheat!(Visuals).draw_fire_tracer(angles);
    }
    call_original!(
        NAME,
        FireBullets,
        weapon,
        player_id,
        origin,
        angles,
        weapon_id,
        mode,
        seed,
        spread,
        damage,
        critical
    );
}
