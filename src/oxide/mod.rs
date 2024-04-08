use std::{error::Error, ffi::CString, mem::transmute};

use libc::{dlclose, dlopen};
use sdl2_sys::SDL_Event;

use crate::{
    d,
    draw::event::Event,
    math::{angles::Angles, vector::Vector3},
    oxide::{cheat::cheats::Cheats, hook::hooks::Hooks, interfaces::Interfaces},
    sdk::{base_client::BaseClient, entity::Entity, global_vars::GlobalVars},
    util::sigscanner::find_sig,
    DRAW,
};

use self::{paint::Paint, entity_cache::EntityCache};

pub mod cheat;
pub mod hook;
pub mod interfaces;
pub mod paint;
pub mod entity_cache;

#[derive(Debug)]
pub struct Oxide<> {
    pub interfaces: Interfaces,
    pub hooks: Hooks,
    pub global_vars: &'static GlobalVars,
    pub cheats: Cheats,
    pub fov: Option<f32>,
    pub get_bone_position_fn: GetBonePositionFn,
    pub last_entity_cache: Option<EntityCache>,
    pub paint: Paint
}
pub type GetBonePositionFn =
    unsafe extern "C-unwind" fn(&Entity, usize, &mut Vector3, &mut Angles) -> ();

impl Oxide {
    pub unsafe fn init() -> Result<Oxide, std::boxed::Box<dyn Error>> {
        let sig =
            "55 89 E5 53 8D 5D ? 83 EC 44 8B 45 ? 89 5C 24 ? 89 44 24 ? 8B 45 ? 89 04 24 E8 ? ? ? ? 8B 45";
        let get_bone_position_fn = transmute(find_sig("./tf/bin/client.so", &sig));
        let interfaces = Interfaces::init()?;
        let hooks = Hooks::init(&interfaces);
        let cheats = Cheats::init();

        let global_vars = Oxide::get_global_vars(interfaces.base_client.interface_ref());

        let paint = Paint::init(&interfaces);

        let oxide = Oxide {
            interfaces,
            cheats,
            hooks,
            global_vars,
            fov: None,
            get_bone_position_fn,
            last_entity_cache: None,
            paint
        };

        Ok(oxide)
    }
    unsafe fn get_global_vars(base_client: &BaseClient) -> &'static mut GlobalVars {
        let hud_update_addr = (*base_client.vmt).hud_update as usize;
        let global_vars: &'static mut &'static mut &'static mut GlobalVars =
            transmute(hud_update_addr + 9);
        **global_vars
    }
    pub fn handle_event(&mut self, raw_event: *mut SDL_Event) {
        let mut event = Event::from(unsafe { *raw_event });

        unsafe {
            if DRAW.is_some() {
                d!().handle_event(&mut event);
            }
            self.cheats.handle_event(&mut event);
            if event.handled {
                (*raw_event).type_ = 0;
            }
        }
    }
    pub fn self_unload() {
        let lib_path = CString::new("/tmp/liboxide.so").unwrap();
        unsafe {
            let handle = dlopen(lib_path.as_ptr(), 6);
            dlclose(handle);
            dlclose(handle);
        }
    }
    pub fn restore(&mut self) {
        self.interfaces.restore();
        self.hooks.restore();
    }

    pub fn global_vars(&self) -> &GlobalVars {
        self.global_vars
    }

    pub fn global_vars_mut(&mut self) -> &mut &'static GlobalVars {
        &mut self.global_vars
    }
}
