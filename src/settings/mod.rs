use std::{fs::create_dir_all, io::Read, path::Path};
use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

use crate::{error::OxideResult, util::dir};

use self::aimbot::AimbotSettings;
use self::crit_manipulation::CritSettings;
use self::movement::MovementSettings;
use self::spread_reduction::SpreadReductionSettings;
use self::visual::VisualSettings;

pub mod aimbot;
pub mod crit_manipulation;
pub mod movement;
pub mod spread_reduction;
pub mod visual;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub aimbot: AimbotSettings,
    pub visual: VisualSettings,
    pub movement: MovementSettings,
    pub spread_reduction: SpreadReductionSettings,
    pub crit_manipulation: CritSettings,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            aimbot: AimbotSettings::new(),
            visual: VisualSettings::new(),
            movement: MovementSettings::new(),
            spread_reduction: SpreadReductionSettings::new(),
            crit_manipulation: CritSettings::new(),
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
