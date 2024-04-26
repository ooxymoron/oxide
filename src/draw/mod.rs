use std::{
    error::Error,
    ffi::CString,
};

use libc::c_void;
use sdl2_sys::*;

use crate::{
    draw::component::{aimbot_fov::AimbotFov, overlay::Overlay, spectator_list::SpectatorList}, log, AUTHOR, NAME, VERSION
};

use self::{
    component::Components,
    event::{Event, EventType},
    fonts::Fonts,
    frame::Frame,
};

pub mod colors;
pub mod component;
pub mod event;
pub mod fonts;
pub mod frame;
pub mod sdl_wrappers;

pub static LOGO: &[u8; 1125850] = include_bytes!("../../assets/oxide-logo-outlined.bmp");

pub struct Draw {
    pub fonts: Fonts,
    pub renderer: *mut SDL_Renderer,
    pub old_ctx: *mut c_void,
    pub ctx: *mut c_void,
    pub components: Components,
    pub cursor: (isize, isize),
    pub logo: *mut SDL_Texture
}

impl Draw {
    pub unsafe fn init(window: *mut SDL_Window) -> Result<Draw, std::boxed::Box<dyn Error>> {
        log!("loading menu");
        let old_ctx = SDL_GL_GetCurrentContext();
        let ctx = SDL_GL_CreateContext(window);
        let mut renderer = SDL_CreateRenderer(window, -1, 0);

        if renderer.is_null() {
            renderer = SDL_GetRenderer(window);
        }

        let title = CString::new(format!(
            "Team Fortress 2 - [{}] v{} by {}",
            NAME, VERSION, AUTHOR
        ))
        .unwrap();

        SDL_SetWindowTitle(window, title.as_ptr());

        SDL_GL_MakeCurrent(window, old_ctx);

        let mut components = Components::new();

        components.add(AimbotFov {});
        components.add(SpectatorList {});
        components.add(Overlay::new());

        let rw = SDL_RWFromMem(LOGO.as_ptr() as *mut c_void, LOGO.len() as i32);
        let bmp = SDL_LoadBMP_RW(rw, 0);
        let logo = SDL_CreateTextureFromSurface(renderer, bmp);

        log!("loaded menu");
        Ok(Draw {
            components,
            fonts: Fonts::init(),
            old_ctx,
            ctx,
            renderer,
            cursor: (0, 0),
            logo
        })
    }

    pub fn restore(&self) {
        unsafe {
            //SDL_DestroyTexture(self.logo);
            SDL_GL_DeleteContext(self.ctx);
        }
        self.fonts.restore();
    }

    pub fn run(&'static mut self, window: *mut SDL_Window) {
        unsafe {
            SDL_GL_MakeCurrent(window, self.ctx);
        }

        let mut frame = Frame::new(window, self.renderer, &mut self.fonts);
        if let Err(e) = self.components.draw(&mut frame, 0, 0) {
            log!("error during drawing {}",e);
        }

        unsafe {
            SDL_RenderPresent(self.renderer);
            SDL_GL_MakeCurrent(window, self.old_ctx);
        }
    }

    pub fn handle_event(&mut self, event: &mut Event) -> bool {
        if let EventType::CursorMove(pos) = event.r#type {
            self.cursor = pos
        }
        self.components.handle_event(event);
        event.handled
    }
}
