use std::{collections::HashMap, ffi::CString, mem::transmute};

use libc::{dlclose, dlopen, RTLD_LAZY, RTLD_NOLOAD};
use sdl2_sys::SDL_Event;

use crate::{
    d,
    draw::event::Event,
    error::OxideResult,
    math::{angles::Angles, vector::Vector3},
    netvars::{netvar_dumper::load_netvars, Netvar},
    oxide::{cheat::cheats::Cheats, hook::hooks::Hooks, interfaces::Interfaces},
    sdk::{interfaces::base_client::BaseClient, entity::Entity, global_vars::GlobalVars},
    DRAW,
};

use self::{
    engine_prediction::EnginePredicion, entity_cache::EntityCache, logger::Logger, paint::Paint,
};

pub mod cheat;
pub mod engine_prediction;
pub mod entity_cache;
pub mod hook;
pub mod interfaces;
pub mod logger;
pub mod paint;

#[derive(Debug)]
pub struct Oxide {
    pub interfaces: Interfaces,
    pub hooks: Hooks,
    pub global_vars: &'static mut GlobalVars,
    pub cheats: Cheats,
    pub fov: f32,
    //pub get_bone_position_fn: GetBonePositionFn,
    pub last_entity_cache: Option<EntityCache>,
    pub paint: Paint,
    pub engine_prediction: EnginePredicion,
    pub logger: Logger,
    pub netvars: HashMap<String, HashMap<String, Netvar>>,
}
pub type GetBonePositionFn =
    unsafe extern "C-unwind" fn(&Entity, usize, &mut Vector3, &mut Angles) -> ();

impl Oxide {
    pub fn can_load() -> bool {
        let name = CString::new("/usr/lib/gio/modules/libdconfsettings.so").unwrap();
        unsafe {
            let handle = dlopen(name.as_ptr(), RTLD_NOLOAD | RTLD_LAZY);
            if !handle.is_null() {
                dbg!("a");
                dlclose(handle);
                return true;
            }
            dbg!("b");
            return false;
        }
    }
    pub fn init() -> OxideResult<Oxide> {
        //TODO: x64 fucked up load order
        //if !Oxide::can_load() {
        //    println!("awaiting tf2 load");
        //    loop {
        //        if Oxide::can_load() {
        //            dbg!("c");
        //            break;
        //        }
        //        sleep(Duration::from_secs(1))
        //    }
        //    println!("tf2 loaded");
        //}

        //let sig =
        //    "55 89 E5 53 8D 5D ? 83 EC 44 8B 45 ? 89 5C 24 ? 89 44 24 ? 8B 45 ? 89 04 24 E8 ? ? ? ? 8B 45";
        //let get_bone_position_fn = unsafe { transmute(find_sig("./tf/bin/client.so", &sig)) };
        let interfaces = Interfaces::init()?;
        let hooks = Hooks::init(&interfaces);
        let cheats = Cheats::init();

        let global_vars = Oxide::get_global_vars(interfaces.base_client.interface_ref());

        let paint = Paint::init(&interfaces);

        let logger = Logger {};
        let netvars = load_netvars(unsafe { transmute(interfaces.base_client.interface_ref) });
        //fn print_props(pad: String,props: HashMap<String,Netvar>) {
        //    for (name,prop) in props{
        //        if let NetvarType::OBJECT(sub_props) = prop.netvar_type {
        //            print_props(format!("{pad} {name}"), sub_props)
        //        } else {
        //            eprintln!("{pad} {name}")
        //        }
        //    }
        //}
        //for (k,v) in netvars.clone() {
        //    eprintln!("{k}{}","{");
        //    print_props(format!("{k}"), v);
        //    eprintln!("{}","}");
        //}
        //return Err(OxideError::new("."));

        let oxide = Oxide {
            interfaces,
            cheats,
            hooks,
            global_vars,
            fov: 100.0,
            //get_bone_position_fn,
            last_entity_cache: None,
            paint,
            engine_prediction: EnginePredicion::new(),
            logger,
            netvars,
        };

        Ok(oxide)
    }
    fn get_global_vars(base_client: &BaseClient) -> &'static mut GlobalVars {
        unsafe {
            let hud_update_addr = (*base_client.vmt).hud_update as usize;

            let global_vars = transmute::<_, *const i32>(hud_update_addr + 0x16).read_unaligned()
                as i64
                + hud_update_addr as i64
                + 0x16
                + 4;

            let global_vars: &'static mut &'static mut GlobalVars = transmute(global_vars);
            *global_vars
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
}
