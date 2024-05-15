use std::ops::{Deref, DerefMut};

use sdl2_sys::SDL_Scancode;
use serde::{de::Error, Deserialize, Serialize};

use super::{sdl_scancode_name_from_string, sdl_scancode_name_to_string};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scancode(pub SDL_Scancode);
impl Scancode {
    pub fn new(code: SDL_Scancode) -> Scancode {
        Scancode(code)
    }
}
impl<'de> Deserialize<'de> for Scancode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Scancode::new(sdl_scancode_name_from_string(
            &String::deserialize(deserializer)?,
        ).map_err(|_|D::Error::custom("invalid keycode"))?))
    }
}
impl Serialize for Scancode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        sdl_scancode_name_to_string(self.0).serialize(serializer)
    }
}
impl Deref for Scancode {
    type Target = SDL_Scancode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Scancode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
