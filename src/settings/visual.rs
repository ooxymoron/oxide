use sdl2_sys::SDL_Scancode;
use serde::{Deserialize, Serialize};

use crate::{draw::component::base::key_input::KeyInputValue, util::{arcm::Arcm, scancode::Scancode}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualSettings {
    pub third_person: Arcm<bool>,
    pub tp_key: Arcm<KeyInputValue>,
    pub tp_offset_key: Arcm<KeyInputValue>,
    pub tp_offset_x: Arcm<f32>,
    pub tp_offset_y: Arcm<f32>,
    pub tp_offset_z: Arcm<f32>,
    pub fov: Arcm<f32>,
    pub esp: Arcm<bool>,
    pub esp_friendlies: Arcm<bool>,
    pub esp_sentreis: Arcm<bool>,
    pub esp_projectiles: Arcm<bool>,
    pub remove_zoom: Arcm<bool>,
    pub remove_scope: Arcm<bool>,
    pub hitboxes: Arcm<bool>,
    pub remove_disguises: Arcm<bool>,
    pub spectator_list: Arcm<bool>,
    pub pure_bypass: Arcm<bool>,
    pub tracers: Arcm<bool>,
    pub impacts: Arcm<bool>,
    pub explosives: Arcm<bool>,
}

impl VisualSettings {
    pub fn new() -> VisualSettings {
        VisualSettings {
            third_person: Arcm::new(false),
            tp_key: Arcm::new(KeyInputValue::Keyboard(Scancode::new(
                SDL_Scancode::SDL_SCANCODE_C,
            ))),
            tp_offset_key: Arcm::new(KeyInputValue::Keyboard(Scancode::new(
                SDL_Scancode::SDL_SCANCODE_T,
            ))),
            tp_offset_x: Arcm::new(0f32),
            tp_offset_y: Arcm::new(0f32),
            tp_offset_z: Arcm::new(0f32),
            fov: Arcm::new(100f32),
            remove_zoom: Arcm::new(false),
            remove_scope: Arcm::new(false),
            esp: Arcm::new(false),
            esp_friendlies: Arcm::new(false),
            esp_sentreis: Arcm::new(false),
            esp_projectiles: Arcm::new(false),
            hitboxes: Arcm::new(false),
            remove_disguises: Arcm::new(false),
            spectator_list: Arcm::new(false),
            pure_bypass: Arcm::new(false),
            tracers: Arcm::new(false),
            impacts: Arcm::new(false),
            explosives: Arcm::new(false),
        }
    }
}
