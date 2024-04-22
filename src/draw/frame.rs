use sdl2_sys::*;

use super::fonts::Fonts;

pub struct Frame {
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
    pub fn window_size(&self) -> (isize, isize) {
        let mut w = 0i32;
        let mut h = 0i32;

        unsafe {
            SDL_GetWindowSize(self.window, &mut w, &mut h);
        }
        return (w as isize, h as isize);
    }
}
