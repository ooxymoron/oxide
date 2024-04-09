use std::{
    error::Error,
    ffi::{CStr, CString},
    mem::transmute,
    thread::sleep,
    time::Duration,
};

use libc::{dlclose, dlopen, RTLD_LAZY, RTLD_NOLOAD};
use sdl2_sys::SDL_Event;

use crate::{
    d,
    draw::event::Event,
    error::OxideError,
    math::{angles::Angles, vector::Vector3},
    oxide::{cheat::cheats::Cheats, hook::hooks::Hooks, interfaces::Interfaces},
    sdk::{
        base_client::BaseClient, entity::Entity, global_vars::GlobalVars, predictions::MoveHelper,
    },
    util::{get_handle, sigscanner::find_sig},
    DRAW,
};

use self::{entity_cache::EntityCache, paint::Paint};

pub mod cheat;
pub mod entity_cache;
pub mod hook;
pub mod interfaces;
pub mod paint;

#[derive(Debug)]
pub struct Oxide {
    pub interfaces: Interfaces,
    pub hooks: Hooks,
    pub global_vars: &'static GlobalVars,
    pub cheats: Cheats,
    pub fov: Option<f32>,
    pub get_bone_position_fn: GetBonePositionFn,
    pub last_entity_cache: Option<EntityCache>,
    pub paint: Paint,
    pub move_helper: Option<&'static MoveHelper>,
}
pub type GetBonePositionFn =
    unsafe extern "C-unwind" fn(&Entity, usize, &mut Vector3, &mut Angles) -> ();

impl Oxide {
    pub fn await_start() {
        loop {
            unsafe {
                let name = CString::new("./bin/vaudio_miles.so").unwrap();
                let handle = dlopen(
                    name.as_ptr(),
                    RTLD_NOLOAD | RTLD_LAZY,
                );
                if !handle.is_null() {
                    dlclose(handle);
                    break;
                }
                sleep(Duration::from_secs(1));
            }
        }
    }
    pub fn init() -> Result<Oxide, std::boxed::Box<dyn Error>> {
        println!("awaiting tf2 load");
        Oxide::await_start();
        println!("tf2 loaded");

        let sig =
            "55 89 E5 53 8D 5D ? 83 EC 44 8B 45 ? 89 5C 24 ? 89 44 24 ? 8B 45 ? 89 04 24 E8 ? ? ? ? 8B 45";
        let get_bone_position_fn = unsafe { transmute(find_sig("./tf/bin/client.so", &sig)) };
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
            paint,
            move_helper: None,
        };

        Ok(oxide)
    }
    fn get_global_vars(base_client: &BaseClient) -> &'static mut GlobalVars {
        unsafe {
            let hud_update_addr = (*base_client.vmt).hud_update as usize;
            let global_vars: &'static mut &'static mut &'static mut GlobalVars =
                transmute(hud_update_addr + 9);
            **global_vars
        }
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
