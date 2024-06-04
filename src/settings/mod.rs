use std::{fs::create_dir_all, io::Read, path::Path};
use std::{fs::File, io::Write};

use sdl2_sys::SDL_Scancode;
use serde::{Deserialize, Serialize};

use crate::{
    draw::component::base::key_input::KeyInputValue,
    error::OxideResult,
    util::{arcm::Arcm, dir, scancode::Scancode},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub aimbot: AimbotSettings,
    pub visual: VisualSettings,
    pub movement: MovementSettings,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            aimbot: AimbotSettings::new(),
            visual: VisualSettings::new(),
            movement: MovementSettings::new(),
        }
    }
    pub fn load() -> OxideResult<Self> {
        let path = format!("{}/main.toml", dir());
        if !Path::new(&path).exists() {
            return Ok(Settings::new());
        }
        let Ok(settings) = (||-> OxideResult<Settings>{
            let mut file = File::open(path)?;
            let mut text = String::new();
            file.read_to_string(&mut text)?;
            Ok(toml::from_str(&text)?)
        })() else {
            return Ok(Settings::new())
        };
        Ok(settings)
    }
    pub fn save(&self) -> OxideResult<()> {
        create_dir_all(dir())?;
        let path = format!("{}/main.toml", dir());
        let mut file = File::create(path)?;
        let text = toml::to_string(self).unwrap();
        file.write_all(text.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AimbotSettings {
    pub enabled: Arcm<bool>,
    pub draw_fov: Arcm<bool>,
    pub fov: Arcm<f32>,
    pub always_on: Arcm<bool>,
    pub key: Arcm<KeyInputValue>,
    pub multipoint: Arcm<bool>,
    pub hitbox_scale: Arcm<f32>,
    pub autoshoot: Arcm<bool>,
    pub silent: Arcm<bool>,
    pub fire_only_when_able: Arcm<bool>,
    pub target_sentries: Arcm<bool>,
    pub target_stickies: Arcm<bool>,
    pub target_invisible: Arcm<bool>,
    pub target_disguised: Arcm<bool>,
    pub wait_for_charge: Arcm<bool>,
    pub baim_if_lethal: Arcm<bool>,
    pub auto_scope: Arcm<bool>,
    pub auto_unscope: Arcm<bool>,
    pub auto_rev: Arcm<bool>,
    pub spread_reduction: Arcm<bool>,
    pub tapfire: Arcm<bool>,
    pub tapfire_only_minigun: Arcm<bool>,
    pub crit_key: Arcm<KeyInputValue>,
}

impl AimbotSettings {
    pub fn new() -> AimbotSettings {
        AimbotSettings {
            enabled: Arcm::new(false),
            draw_fov: Arcm::new(false),
            fov: Arcm::new(30.0),
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
            target_invisible: Arcm::new(false),
            target_disguised: Arcm::new(false),
            target_stickies: Arcm::new(false),
            wait_for_charge: Arcm::new(false),
            baim_if_lethal: Arcm::new(false),
            auto_scope: Arcm::new(false),
            auto_unscope: Arcm::new(false),
            auto_rev: Arcm::new(false),
            spread_reduction: Arcm::new(false),
            tapfire: Arcm::new(false),
            tapfire_only_minigun: Arcm::new(false),
            crit_key: Arcm::new(KeyInputValue::Keyboard(Scancode::new(
                SDL_Scancode::SDL_SCANCODE_RIGHT,
            ))),
        }
    }
}

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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSettings {
    pub bhop: Arcm<bool>,
    pub autostrafe: Arcm<bool>,
    pub no_push: Arcm<bool>,
    pub momentum_compensation: Arcm<bool>,
}

impl MovementSettings {
    pub fn new() -> MovementSettings {
        MovementSettings {
            bhop: Arcm::new(false),
            autostrafe: Arcm::new(false),
            no_push: Arcm::new(false),
            momentum_compensation: Arcm::new(false),
        }
    }
}
