use crate::{
    call_original, cfn,
    math::{angles::Angles, vector::Vector3},
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
    u32,
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
    seed: u32,
    spread: f32,
    damage: f32,
    critical: bool,
) {
    call_original!(NAME, FireBullets, 
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
