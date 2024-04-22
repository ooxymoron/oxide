
use std::mem::transmute;

use sdl2_sys::*;


#[derive(Debug,Clone)]
pub enum EventType {
    CursorMove((isize, isize)),
    MouseButtonDown,
    MouseButtonUp,
    KeyDown(SDL_Scancode),
    KeyUp(SDL_Scancode),
    None,
}
#[derive(Debug,Clone)]
pub struct Event {
    pub handled: bool,
    pub r#type: EventType
}

impl From<SDL_Event> for Event {
    fn from(event: SDL_Event) -> Event {
        let r#type = unsafe {
            match transmute::<u32, SDL_EventType>(event.type_) {
                SDL_EventType::SDL_MOUSEMOTION => {
                    let motion = (event).motion;
                    EventType::CursorMove((motion.x as isize, motion.y as isize))
                }
                SDL_EventType::SDL_MOUSEBUTTONDOWN => EventType::MouseButtonDown,
                SDL_EventType::SDL_MOUSEBUTTONUP => EventType::MouseButtonUp,
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
            r#type
        }
    }
}
