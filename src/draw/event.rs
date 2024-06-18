use std::mem::transmute;

use sdl2_sys::*;

use super::component::base::key_input::KeyInputValue;

#[derive(Debug, Clone)]
pub enum EventType {
    CursorMove((isize, isize)),
    MouseButtonDown(u8),
    MouseButtonUp(u8),
    KeyDown(SDL_Scancode),
    KeyUp(SDL_Scancode),
    None,
}
#[derive(Debug, Clone)]
pub struct Event {
    pub handled: bool,
    pub r#type: EventType,
}

impl From<SDL_Event> for Event {
    fn from(event: SDL_Event) -> Event {
        let r#type = unsafe {
            match transmute::<u32, SDL_EventType>(event.type_) {
                SDL_EventType::SDL_MOUSEMOTION => {
                    let motion = (event).motion;
                    EventType::CursorMove((motion.x as isize, motion.y as isize))
                }
                SDL_EventType::SDL_MOUSEBUTTONDOWN => {
                    EventType::MouseButtonDown(event.button.button)
                }
                SDL_EventType::SDL_MOUSEBUTTONUP => EventType::MouseButtonUp(event.button.button),
                SDL_EventType::SDL_KEYDOWN => {
                    let key = event.key.keysym.scancode;
                    EventType::KeyDown(key)
                }
                SDL_EventType::SDL_KEYUP => {
                    let key = event.key.keysym.scancode;
                    EventType::KeyUp(key)
                }
                _ => EventType::None,
            }
        };
        Event {
            handled: false,
            r#type,
        }
    }
}

impl Event {
    pub fn is_key_up(&self, target_key: &KeyInputValue) -> bool {
        match target_key {
            KeyInputValue::Keyboard(target_key) => match self.r#type {
                EventType::KeyUp(key) => {
                    if key == target_key.0 {
                        return true;
                    }
                }
                _ => {}
            },
            KeyInputValue::Mouse(target_key) => match self.r#type {
                EventType::MouseButtonUp(key) => {
                    if key == *target_key {
                        return true;
                    }
                }
                _ => (),
            },
            KeyInputValue::None => {}
        }
        false
    }
    pub fn is_key_down(&self, target_key: &KeyInputValue) -> bool {
        match target_key {
            KeyInputValue::Keyboard(target_key) => match self.r#type {
                EventType::KeyDown(key) => {
                    if key == target_key.0 {
                        return true;
                    }
                }
                _ => {}
            },
            KeyInputValue::Mouse(target_key) => match self.r#type {
                EventType::MouseButtonDown(key) => {
                    if key == *target_key {
                        return true;
                    }
                }
                _ => (),
            },
            KeyInputValue::None => {}
        }
        false
    }
}
