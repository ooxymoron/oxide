use sdl2_sys::SDL_Scancode;
use serde::{Deserialize, Serialize};

use crate::{draw::component::base::key_input::KeyInputValue, util::{arcm::Arcm, scancode::Scancode}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CritSettings {
    pub key: Arcm<KeyInputValue>,
    pub auto_cycle_rapid_fire: Arcm<bool>,
}

impl CritSettings {
    pub fn new() -> CritSettings {
        CritSettings {
            key: Arcm::new(KeyInputValue::Keyboard(Scancode::new(
                SDL_Scancode::SDL_SCANCODE_RIGHT,
            ))),
            auto_cycle_rapid_fire: Arcm::new(false),
        }
    }
}
