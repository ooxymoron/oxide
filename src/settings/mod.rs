#[allow(deprecated)]
use std::{env::home_dir, fs::File, io::Write};
use std::{fs::create_dir_all, io::Read, path::Path};

use sdl2_sys::SDL_Scancode;
use serde::{Deserialize, Serialize};

use crate::{
    error::OxideResult,
    util::{arcm::Arcm, scancode::Scancode},
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
        let path = format!("{}/main.toml", Settings::dir());
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
    fn dir() -> String {
        #[allow(deprecated)]
        return format!("{}/.config/oxide/", home_dir().unwrap().to_string_lossy());
    }
    pub fn save(&self) -> OxideResult<()> {
        create_dir_all(Settings::dir())?;
        let path = format!("{}/main.toml", Settings::dir());
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
    pub key: Arcm<Scancode>,
    pub multipoint: Arcm<bool>,
    pub hitbox_scale: Arcm<f32>,
    pub autoshoot: Arcm<bool>,
    pub silent: Arcm<bool>,
    pub target_sentries: Arcm<bool>,
    pub target_stickies: Arcm<bool>,
    pub target_invisible: Arcm<bool>,
    pub target_disguised: Arcm<bool>,
    pub ambasador_wait_for_hs: Arcm<bool>,
    pub baim_if_lethal: Arcm<bool>,
    pub engine_prediction: Arcm<bool>
}

impl AimbotSettings {
    pub fn new() -> AimbotSettings {
        AimbotSettings {
            enabled: Arcm::new(false),
            draw_fov: Arcm::new(false),
            fov: Arcm::new(30.0),
            key: Arcm::new(Scancode::new(SDL_Scancode::SDL_SCANCODE_LSHIFT)),
            multipoint: Arcm::new(false),
            hitbox_scale: Arcm::new(0.8),
            autoshoot: Arcm::new(false),
            silent: Arcm::new(false),
            target_sentries: Arcm::new(false),
            target_invisible: Arcm::new(false),
            target_disguised: Arcm::new(false),
            target_stickies: Arcm::new(false),
            ambasador_wait_for_hs: Arcm::new(false),
            baim_if_lethal: Arcm::new(false),
            engine_prediction: Arcm::new(true)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualSettings {
    pub third_person: Arcm<bool>,
    pub third_person_key: Arcm<Scancode>,
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
}

impl VisualSettings {
    pub fn new() -> VisualSettings {
        VisualSettings {
            third_person: Arcm::new(false),
            third_person_key: Arcm::new(Scancode::new(SDL_Scancode::SDL_SCANCODE_C)),
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSettings {
    pub bhop: Arcm<bool>,
    pub autostrafe: Arcm<bool>,
}

impl MovementSettings {
    pub fn new() -> MovementSettings {
        MovementSettings {
            bhop: Arcm::new(false),
            autostrafe: Arcm::new(false),
        }
    }
}
