use std::ffi::c_char;

const MAX_WEAPON_STRING: usize = 80;
const MAX_WEAPON_PREFIX: usize = 16;
const MAX_WEAPON_AMMO_NAME: usize = 32;
const MAX_SHOOT_SOUNDS: usize = 16;

pub enum WeaponSound {
    Empty,
    Single,
    SingleNpc,
    WpnDouble, // Can't be "DOUBLE" because windows.h uses it.
    DoubleNpc,
    Burst,
    Reload,
    ReloadNpc,
    MeleeMiss,
    MeleeHit,
    MeleeHitWorld,
    Special1,
    Special2,
    Special3,
    Taunt,
    Deploy,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WeaponInfo {
    pub pad: [u8;8],
    pub parsed_script: bool,
    pub loaded_hud_elements: bool,
    pub class_name: [c_char; MAX_WEAPON_STRING],
    pub print_name: [c_char; MAX_WEAPON_STRING],
    pub view_model: [c_char; MAX_WEAPON_STRING],
    pub world_model: [c_char; MAX_WEAPON_STRING],
    pub animation_prefix: [c_char; MAX_WEAPON_PREFIX],
    pub slot: i32,
    pub position: i32,
    pub max_clip1: i32,
    pub max_clip2: i32,
    pub default_clip1: i32,
    pub default_clip2: i32,
    pub weight: i32,
    pub rumble_effect: i32,
    pub auto_switch_to: bool,
    pub auto_switch_from: bool,
    pub flags: i32,
    pub ammo1: [c_char; MAX_WEAPON_AMMO_NAME],
    pub ammo2: [c_char; MAX_WEAPON_AMMO_NAME],
    pub shoot_sounds: [[c_char; MAX_WEAPON_STRING]; MAX_SHOOT_SOUNDS],
    pub ammo_type: i32,
    pub ammo2_type: i32,
    pub melee_weapon: bool,
    pub built_right_handed: bool,
    pub allow_flipping: bool,
    pub sprite_count: i32,
    pub icon_active: *const HudTexture,
    pub icon_inactive: *const HudTexture,
    pub icon_ammo: *const HudTexture,
    pub icon_ammo2: *const HudTexture,
    pub icon_crosshair: *const HudTexture,
    pub icon_autoaim: *const HudTexture,
    pub icon_zoomed_crosshair: *const HudTexture,
    pub icon_zoomed_autoaim: *const HudTexture,
    pub icon_small: *const HudTexture,
    pub show_usage_hint: bool,
    pub weapon_dat: [WeaponData; 2],
    pub weapon_type: i32,
    pub grenade: bool,
    pub damage_radius: f32,
    pub pimer_time: f32,
    pub lower_weapon: bool,
    pub suppress_gren_timer: bool,
    pub has_team_skins_viewmodel: bool,
    pub has_team_skins_worldmodel: bool,
    pub muzzle_flash_model: [c_char; 128],
    pub muzzle_flash_model_duration: f32,
    pub muzzle_flash_particle_effect: [c_char; 128],
    pub tracer_efect: [c_char; 128],
    pub do_instant_eject_brass: bool,
    pub brass_model: [c_char; 128],
    pub explosion_sound: [c_char; 128],
    pub explosion_effect: [c_char; 128],
    pub explosion_player_effect: [c_char; 128],
    pub explosion_water_effect: [c_char; 128],
    pub dont_drop: bool,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WeaponData {
    pub damage: i32,
    pub bullets_per_shot: i32,
    pub range: f32,
    pub spread: f32,
    pub punch_angle: f32,
    pub time_fire_delay: f32,
    pub time_idle: f32,
    pub time_idle_empty: f32,
    pub time_reload_start: f32,
    pub time_reload: f32,
    pub draw_crosshair: bool,
    pub projectile: i32,
    pub ammo_per_shot: i32,
    pub projectile_speed: f32,
    pub smack_delay: f32,
    pub use_rapid_fire_crits: bool,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HudTexture {
    pub short_name: [c_char; 64],
    pub texture_file: [c_char; 64],
    pub render_using_font: bool,
    pub precached: bool,
    pub character_in_font: char,
    pub font: u32,
    pub texture_id: i32,
    pub tex_coords: [f32; 4],
    pub rc: [i32; 4],
}
