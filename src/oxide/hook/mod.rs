use std::{
    collections::HashMap,
    ffi::CString,
    mem::{transmute, ManuallyDrop},
};

use libc::dlsym;

use crate::{
    oxide::hook::{
        create_move::CreateMoveHook, frame_stage_notify::FrameStageNotifyHook,
        override_view::OverrideViewHook, paint::PaintHook, paint_traverse::PaintTraverseHook,
        poll_event::PollEventHook, run_command::RunCommandHook, swap_window::SwapWindowHook,
    },
    util::{get_handle, sigscanner::find_sig},
};

use self::detour::DetourHook;

use super::interfaces::Interfaces;

pub mod base_interpolate_part1;
pub mod create_move;
pub mod detour;
pub mod fire_event;
pub mod frame_stage_notify;
pub mod level_shutdown;
pub mod load_whitelist;
pub mod override_view;
pub mod paint;
pub mod paint_traverse;
pub mod pointer_hook;
pub mod poll_event;
pub mod run_command;
pub mod swap_window;

pub trait Hook: std::fmt::Debug {
    fn restore(&mut self);
}

#[derive(Debug)]
pub struct Hooks {
    ptr_hooks: HashMap<String, Box<dyn Hook + 'static>>,
    pub detour_hooks: HashMap<String, DetourHook>,
}

impl Hooks {
    pub fn init(interfaces: &Interfaces) -> Hooks {
        let mut ptr_hooks = HashMap::new();
        let mut tramp_hooks = HashMap::new();
        macro_rules! InitVmtHook {
            ($HookClass:ident,$val:expr) => {
                ptr_hooks.insert(
                    $HookClass::name(),
                    Box::new($HookClass::init($val)) as Box<dyn Hook>,
                );
            };
        }

        InitVmtHook!(
            OverrideViewHook,
            &(*interfaces.client_mode.get_vmt()).override_view
        );
        InitVmtHook!(
            FrameStageNotifyHook,
            &(*interfaces.base_client.get_vmt()).frame_stage_notify
        );
        InitVmtHook!(
            PaintTraverseHook,
            &(*interfaces.panel.get_vmt()).paint_traverse
        );
        InitVmtHook!(PaintHook, &(*interfaces.engine_vgui.get_vmt()).paint);
        InitVmtHook!(
            CreateMoveHook,
            &(*interfaces.client_mode.get_vmt()).create_move
        );
        InitVmtHook!(
            RunCommandHook,
            &(*interfaces.prediction.get_vmt()).run_command
        );
        //load whitelist
        //55 48 89 E5 41 55 41 54 49 89 FC 48 83 EC 60

        tramp_hooks.insert(
            load_whitelist::NAME.to_string(),
            DetourHook::hook(
                find_sig(
                    "./bin/linux64/engine.so",
                    "55 48 89 E5 41 55 41 54 49 89 FC 48 83 EC 60",
                )
                .unwrap(),
                load_whitelist::load_whitelist_hook as *const u8,
            ),
        );

        tramp_hooks.insert(
            fire_event::NAME.to_string(),
            DetourHook::hook(
                find_sig(
                    "./bin/linux64/engine.so",
                    "55 48 89 E5 41 57 41 56 41 55 41 54 53 48 81 EC 88 00 00 00 48 85 F6",
                )
                .unwrap(),
                fire_event::load_whitelist_hook as *const u8,
            ),
        );

        //tramp_hooks.insert(
        //    base_interpolate_part1::NAME.to_string(),
        //    DetourHook::hook(
        //        find_sig(
        //            "./tf/bin/client.so",
        //            "55 89 E5 57 56 53 83 EC 2C 8B 45 ? 8B 5D ? 8B 75 ? 8B 7D ? C7 00 01 00 00 00",
        //        ),
        //        base_interpolate_part1::BaseInterpolatePart1Hook as *const u8,
        //    ),
        //);

        unsafe {
            let handle = get_handle("/usr/lib/libSDL2-2.0.so.0").unwrap();
            let name = CString::new("SDL_GL_SwapWindow").unwrap();
            let exprted_fn: *const u8 = transmute(dlsym(handle, name.as_ptr()));
            let jump_dist = (exprted_fn.byte_add(6) as *const i32).read() as usize;
            let swap_window_ptr = exprted_fn.byte_add(6 + jump_dist + 4);
            InitVmtHook!(SwapWindowHook, transmute(swap_window_ptr));

            let name = CString::new("SDL_PollEvent").unwrap();
            let exprted_fn: *const u8 = transmute(dlsym(handle, name.as_ptr()));
            let jump_dist = (exprted_fn.byte_add(6) as *const i32).read() as usize;
            let poll_event_ptr = exprted_fn.byte_add(6 + jump_dist + 4);
            InitVmtHook!(PollEventHook, transmute(poll_event_ptr));
        }

        Hooks {
            ptr_hooks,
            detour_hooks: tramp_hooks,
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
