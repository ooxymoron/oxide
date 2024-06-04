use std::{collections::HashMap, ffi::CString, mem::transmute, process::Command};

use libc::{dlclose, dlopen, RTLD_LAZY, RTLD_NOLOAD};
use sdl2_sys::SDL_Event;

use crate::{
    d,
    draw::event::Event,
    error::OxideResult,
    math::{angles::Angles, vector3::Vector3},
    netvars::{netvar_dumper::load_netvars, Netvar},
    oxide::{cheat::cheats::Cheats, hook::Hooks, interfaces::Interfaces},
    sdk::{
        client_state::ClientState,
        entity::Entity,
        event_manager::GameEventManager,
        global_vars::GlobalVars,
        interfaces::{base_client::BaseClient, base_engine::BaseEngine},
    },
    util::{
        handles::{CLIENT, ENGINE, SDL},
        sigscanner::find_sig,
    },
    DRAW,
};

use self::{
    engine_prediction::EnginePredicion, entity_cache::EntityCache, logger::Logger, paint::Paint, player_db::PlayerDb, player_resource_manager::PlayerResourceManager, util::Util
};

pub mod cheat;
pub mod engine_prediction;
pub mod entity_cache;
pub mod hook;
pub mod interfaces;
pub mod logger;
pub mod paint;
pub mod player_resource_manager;
pub mod util;
pub mod player_db;

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
    pub unloading: bool,
    pub util: Util,
    pub client_state: &'static mut ClientState,
    pub event_manager: &'static mut GameEventManager,
    pub player_resource_manager: PlayerResourceManager,
    pub prediction_seed: &'static mut i32,
    pub player_db: PlayerDb
}
pub type GetBonePositionFn =
    unsafe extern "C-unwind" fn(&Entity, usize, &mut Vector3, &mut Angles) -> ();

impl Oxide {
    pub fn can_load() -> bool {
        let name = CString::new("/usr/lib/gio/modules/libdconfsettings.so").unwrap();
        unsafe {
            let handle = dlopen(name.as_ptr(), RTLD_NOLOAD | RTLD_LAZY);
            if !handle.is_null() {
                dlclose(handle);
                return true;
            }
            return false;
        }
    }
    pub fn init() -> OxideResult<Oxide> {
        let output = Command::new("ldconfig").arg("-p").output()?;
        let output = String::from_utf8_lossy(&output.stdout);
        let sdl_path = output
            .split("\n")
            .filter(|line| line.find("libSDL2-2.0.so.0").is_some() && line.find("64").is_some())
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap();
        unsafe { SDL = Box::leak(sdl_path.to_string().into_boxed_str()) }

        let interfaces = Interfaces::init()?;
        let hooks = Hooks::init();
        let cheats = Cheats::init();

        let global_vars = Oxide::get_global_vars(interfaces.base_client.interface_ref());
        let client_state =
            Oxide::get_client_state(unsafe { transmute(interfaces.base_engine.interface_ref()) });
        let event_manager = Oxide::get_event_manager();
        let prediction_seed = Oxide::get_prediction_seed();

        let netvars = load_netvars(unsafe { transmute(interfaces.base_client.interface_ref()) });
        let paint = Paint::init(&interfaces);

        let logger = Logger {};

        let oxide = Oxide {
            interfaces,
            cheats,
            hooks,
            global_vars,
            fov: 100.0,
            last_entity_cache: None,
            paint,
            engine_prediction: EnginePredicion::new(),
            logger,
            netvars,
            unloading: false,
            util: Util::init(),
            client_state,
            event_manager,
            player_resource_manager: PlayerResourceManager::new(),
            prediction_seed,
            player_db: PlayerDb::init()
        };

        Ok(oxide)
    }
    fn get_client_state(base_engine: &BaseEngine) -> &'static mut ClientState {
        unsafe {
            let server_cmd_key_values = base_engine.vmt.read().server_cmd_key_values as *const u8;
            transmute(
                transmute::<_, *const u32>(server_cmd_key_values)
                    .byte_add(3)
                    .read() as u64
                    + transmute::<_, u64>(server_cmd_key_values),
            )
        }
    }
    fn get_event_manager() -> &'static mut GameEventManager {
        unsafe {
            let addr = find_sig::<u32>(ENGINE, "55 48 8D 3D ? ? ? ? 48 89 E5 E8 ? ? ? ? 48 8D 15 ? ? ? ? 48 8D 35 ? ? ? ? 48 8D 3D ? ? ? ? E8 ? ? ? ? 4C 8D 05").unwrap();
            let game_event_manager =
                transmute(addr.byte_add(4).read() as u64 + transmute::<_, u64>(addr) + 8);
            game_event_manager
        }
    }
    fn get_prediction_seed() -> &'static mut i32 {
        unsafe {
            let set_pred_cmd = find_sig::<u32>(CLIENT, "48 85 FF 74 ? 8B 57 ? 48 8D 05").unwrap();
            let pred_sed = transmute(
                set_pred_cmd.byte_add(11).read() as u64
                    + transmute::<_, u64>(set_pred_cmd)
                    + 11
                    + 4,
            );
            pred_sed
        }
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
