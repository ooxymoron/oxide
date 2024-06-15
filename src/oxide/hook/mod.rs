use std::{
    collections::HashMap,
    ffi::CString,
    mem::{transmute, ManuallyDrop},
};

use libc::dlsym;

use crate::{
    o,
    oxide::hook::{
        create_move::CreateMoveHook, dispatch_user_message::DispatchUserMessageHook,
        fire_event::FireEventHook, frame_stage_notify::FrameStageNotifyHook,
        level_shutdown::LevelShutdownHook, paint::PaintHook, paint_traverse::PaintTraverseHook,
        poll_event::PollEventHook, pre_render::PreRenderHook, run_command::RunCommandHook,
        should_draw_local_player::ShouldDrawLocalPlayerHook,
        should_draw_view_model::ShouldDrawViewModelHook, swap_window::SwapWindowHook,
    },
    sdk::HasVmt,
    util::{get_handle, handles::*, sigscanner::find_sig},
};

use self::detour::DetourHook;

pub mod add_to_tail_server;
pub mod base_interpolate_part1;
pub mod cl_send_move;
pub mod create_move;
pub mod detour;
pub mod dispatch_effect;
pub mod dispatch_user_message;
pub mod fire_bullet;
pub mod fire_bullets;
pub mod fire_bullets_server;
pub mod fire_event;
pub mod frame_stage_notify;
pub mod get_user_cmd;
pub mod level_shutdown;
pub mod load_whitelist;
pub mod paint;
pub mod paint_traverse;
pub mod plat_floattime;
pub mod pointer_hook;
pub mod poll_event;
pub mod pre_render;
pub mod process_user_cmds;
pub mod run_command;
pub mod send_perf_server;
pub mod send_user_msg;
pub mod should_draw_local_player;
pub mod should_draw_view_model;
pub mod swap_window;
pub mod validate_user_cmd;
pub mod write_user_cmd;
pub mod write_user_cmd_delta_to_buffer;
pub mod calc_is_attack_critical;
pub mod calc_is_attack_critical_melee;

pub trait Hook: std::fmt::Debug {
    fn restore(&mut self);
}

#[derive(Debug)]
pub struct Hooks {
    ptr_hooks: HashMap<String, Box<dyn Hook + 'static>>,
    pub detour_hooks: HashMap<String, DetourHook>,
}

impl Hooks {
    pub fn init() -> Hooks {
        Hooks {
            ptr_hooks: HashMap::new(),
            detour_hooks: HashMap::new(),
        }
    }
    pub fn init_hooks(&mut self) {
        let interfaces = &o!().interfaces;
        macro_rules! InitPointerHook {
            ($HookClass:ident,$val:expr) => {
                self.ptr_hooks.insert(
                    $HookClass::name(),
                    Box::new($HookClass::init($val)) as Box<dyn Hook>,
                );
            };
        }
        macro_rules! InitDetourHook {
            ($hook:ident,$module:expr,$sig:expr) => {
                self.detour_hooks.insert(
                    $hook::NAME.to_string(),
                    DetourHook::hook(find_sig($module, $sig).unwrap(), $hook::hook as *const u8),
                );
            };
        }

        InitPointerHook!(FireEventHook, &o!().event_manager.get_vmt().fire_event);
        InitPointerHook!(
            PreRenderHook,
            &interfaces.client_mode.get_vmt().override_view
        );
        InitPointerHook!(
            LevelShutdownHook,
            &interfaces.base_client.get_vmt().level_shutdown
        );
        InitPointerHook!(
            ShouldDrawLocalPlayerHook,
            &interfaces.client_mode.get_vmt().should_draw_entity
        );
        InitPointerHook!(
            ShouldDrawViewModelHook,
            &interfaces.client_mode.get_vmt().should_draw_view_model
        );
        InitPointerHook!(
            FrameStageNotifyHook,
            &interfaces.base_client.get_vmt().frame_stage_notify
        );
        InitPointerHook!(
            PaintTraverseHook,
            &interfaces.panel.get_vmt().paint_traverse
        );
        InitPointerHook!(
            CreateMoveHook,
            &interfaces.client_mode.get_vmt().create_move
        );
        InitPointerHook!(RunCommandHook, &interfaces.prediction.get_vmt().run_command);
        InitDetourHook!(
            get_user_cmd,
            CLIENT,
            "48 63 C6 89 F2 48 69 C0 B7 60 0B B6 C1 FA 1F 48 C1 E8 20 01 F0 C1 F8 06 29 D0 89 F2 6B C0 5A 29 C2 48 63 C2 48 8D 14 ? 48 8B 87 ? ? ? ? 48 8D 04"
        );
        InitDetourHook!(
            load_whitelist,
            ENGINE,
            "55 48 89 E5 41 55 41 54 49 89 FC 48 83 EC 60"
        );
        InitDetourHook!(fire_bullets, CLIENT, "55 48 89 E5 41 57 49 89 FF 44 89 C7");
        InitDetourHook!(
            fire_bullet,
            CLIENT,
            "55 48 89 E5 41 57 49 89 D7 41 56 41 55 49 89 FD 41 54 49 89 F4"
        );

        //INFO: PAINT HOOK NEEDS TO LOAD AFTER DISPATCH USER MESSAGE HOOK
        InitPointerHook!(
            DispatchUserMessageHook,
            &interfaces.base_client.get_vmt().dispatch_user_message
        );
        InitPointerHook!(PaintHook, &interfaces.engine_vgui.get_vmt().paint);
        InitDetourHook!(
            add_to_tail_server,
            SERVER,
            "55 48 89 E5 41 56 49 89 FE 41 55 41 89 F5 41 54 49 89 D4"
        );
        InitDetourHook!(
            process_user_cmds,
            SERVER,
            "55 48 89 E5 41 57 41 56 45 89 CE 41 55 49 89 F5"
        );

        InitDetourHook!(
            cl_send_move,
            ENGINE,
            "55 66 0F EF C0 48 89 E5 41 57 41 56 48 8D BD"
        );
        InitDetourHook!(
            validate_user_cmd,
            CLIENT,
            "55 48 89 E5 41 56 41 89 D6 41 55 49 89 FD 41 54 4C 8D 65"
        );
        InitDetourHook!(
            write_user_cmd,
            CLIENT,
            "55 48 89 E5 41 55 49 89 D5 41 54 49 89 FC 53 48 89 F3 48 83 EC 08 8B 72"
        );

        InitDetourHook!(
            write_user_cmd_delta_to_buffer,
            CLIENT,
            "55 48 8D 05 ? ? ? ? 66 0F EF C0 48 89 E5 41 57 49 89 FF"
        );
        InitDetourHook!(
            calc_is_attack_critical,
            CLIENT,
            "55 48 89 E5 41 57 41 56 41 55 41 54 49 89 FC 53 48 83 EC 28 E8 ? ? ? ? 48 85 C0 0F 84 ? ? ? ? 48 8D 15"
        );
        InitDetourHook!(
            calc_is_attack_critical_melee,
            CLIENT,
            "55 48 89 E5 41 57 41 56 41 55 41 54 49 89 FC 53 48 83 EC 18 E8 ? ? ? ? 48 85 C0 74 ? 48 89 C3 48 8B 00 48 89 DF FF 90 ? ? ? ? 84 C0"
        );



        unsafe {
            let handle = get_handle(SDL).unwrap();
            let name = CString::new("SDL_GL_SwapWindow").unwrap();
            let exprted_fn: *const u8 = transmute(dlsym(handle, name.as_ptr()));
            let jump_dist = (exprted_fn.byte_add(6) as *const i32).read() as usize;
            let swap_window_ptr = exprted_fn.byte_add(6 + jump_dist + 4);
            InitPointerHook!(SwapWindowHook, transmute(swap_window_ptr));

            let name = CString::new("SDL_PollEvent").unwrap();
            let exprted_fn: *const u8 = transmute(dlsym(handle, name.as_ptr()));
            let jump_dist = (exprted_fn.byte_add(6) as *const i32).read() as usize;
            let poll_event_ptr = exprted_fn.byte_add(6 + jump_dist + 4);
            InitPointerHook!(PollEventHook, transmute(poll_event_ptr));
        }
    }
    pub fn get<T>(&mut self, name: String) -> ManuallyDrop<&mut Box<T>> {
        unsafe { ManuallyDrop::new(transmute(self.ptr_hooks.get_mut(&name).unwrap())) }
    }
    pub fn restore(&mut self) {
        for (_, hook) in &mut self.ptr_hooks {
            hook.restore()
        }
        for (_, hook) in &mut self.detour_hooks {
            hook.restore()
        }
    }
}
