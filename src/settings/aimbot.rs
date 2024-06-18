use sdl2_sys::SDL_Scancode;
use serde::{Deserialize, Serialize};

use crate::{draw::component::base::key_input::KeyInputValue, sdk::entity::hitbox::PlayerHitboxId, util::{arcm::Arcm, scancode::Scancode}};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AimbotSettings {
    pub enabled: Arcm<bool>,
    pub draw_fov: Arcm<bool>,
    pub fov: Arcm<f32>,
    pub hitboxes: Arcm<Vec<PlayerHitboxId>>,
    pub target_persistance_duration: Arcm<f32>,
    pub always_on: Arcm<bool>,
    pub key: Arcm<KeyInputValue>,
    pub multipoint: Arcm<bool>,
    pub hitbox_scale: Arcm<f32>,
    pub autoshoot: Arcm<bool>,
    pub silent: Arcm<bool>,
    pub fire_only_when_able: Arcm<bool>,
    pub target_sentries: Arcm<bool>,
    pub target_buildings: Arcm<bool>,
    pub target_stickies: Arcm<bool>,
    pub target_invisible: Arcm<bool>,
    pub target_disguised: Arcm<bool>,
    pub wait_for_charge: Arcm<bool>,
    pub baim_if_lethal: Arcm<bool>,
    pub auto_scope: Arcm<bool>,
    pub auto_unscope: Arcm<bool>,
    pub auto_rev: Arcm<bool>,
}

impl AimbotSettings {
    pub fn new() -> AimbotSettings {
        AimbotSettings {
            enabled: Arcm::new(false),
            draw_fov: Arcm::new(false),
            fov: Arcm::new(30.0),
            hitboxes: Arcm::new(PlayerHitboxId::all()),
            target_persistance_duration: Arcm::new(1.0),
            always_on: Arcm::new(false),
            key: Arcm::new(KeyInputValue::Keyboard(Scancode::new(
                SDL_Scancode::SDL_SCANCODE_LSHIFT,
            ))),
            multipoint: Arcm::new(false),
            hitbox_scale: Arcm::new(0.8),
            autoshoot: Arcm::new(false),
            silent: Arcm::new(false),
            fire_only_when_able: Arcm::new(false),
            target_sentries: Arcm::new(false),
            target_buildings: Arcm::new(false),
            target_invisible: Arcm::new(false),
            target_disguised: Arcm::new(false),
            target_stickies: Arcm::new(false),
            wait_for_charge: Arcm::new(false),
            baim_if_lethal: Arcm::new(false),
            auto_scope: Arcm::new(false),
            auto_unscope: Arcm::new(false),
            auto_rev: Arcm::new(false),
        }
    }
}
