use sdl2_sys::*;

use super::fonts::Fonts;

pub struct Frame {
    #[allow(dead_code)]
    window: *mut SDL_Window,
    pub renderer: *mut SDL_Renderer,
    pub fonts: &'static mut Fonts 
}
impl Frame {
    pub fn new(window: *mut SDL_Window, renderer: *mut SDL_Renderer,fonts: &'static mut Fonts) -> Frame {
        Frame{
            window,
            renderer,
            fonts
        }
    }
}
