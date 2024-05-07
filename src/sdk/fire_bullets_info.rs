use crate::math::vector3::Vector3;

use super::interfaces::Entity;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FireBulletsInfo {
    pub shots: u32,
    pub src: Vector3,
    pub dir_shooting: Vector3,
    pub vec_spread: Vector3,
    pub distance: f32,
    pub ammo_type: i32,
    pub tracer_freq: i32,
    pub damage: f32,
    pub player_damage: i32, // Damage to be used instead of m_flDamage if we hit a player
    pub flags: i32,        // See FireBulletsFlags_t
    pub damage_force_scale: f32,
    pub attacker: *mut Entity,
    pub additional_ignore_ent:*mut Entity,
    pub primary_attack: bool,
    pub use_server_random_seed: bool,
}
