use std::{
    collections::HashMap,
    ffi::CString,
    mem::{transmute, ManuallyDrop},
};

use libc::{c_void, dlsym};

use crate::{
    oxide::{
        hook::{base_interpolate_part1, detour::DetourHook, run_command::RunCommandHook},
        interfaces::Interfaces,
    },
    util::{debug::print_bytes, get_handle, sigscanner::find_sig},
};

use super::{
    create_move::CreateMoveHook, frame_stage_notify::FrameStageNotifyHook,
    override_view::OverrideViewHook, paint::PaintHook, paint_traverse::PaintTraverseHook,
    poll_event::PollEventHook, swap_window::SwapWindowHook, Hook,
};

#[derive(Debug)]
pub struct Hooks {
    ptr_hooks: HashMap<String, Box<dyn Hook + 'static>>,
    pub detour_hooks: HashMap<String, DetourHook>,
}

macro_rules! InitVmtHook {
    ($hooks:expr,$HookClass:ident,$val:expr) => {
        $hooks.insert(
            $HookClass::name(),
            Box::new($HookClass::init($val)) as Box<dyn Hook>,
        );
    };
}

impl Hooks {
    pub fn init(interfaces: &Interfaces) -> Hooks {
        let mut ptr_hooks = HashMap::new();
        let mut tramp_hooks = HashMap::new();

        InitVmtHook!(
            ptr_hooks,
            OverrideViewHook,
            &(*interfaces.client_mode.get_vmt()).override_view
        );
        InitVmtHook!(
            ptr_hooks,
            FrameStageNotifyHook,
            &(*interfaces.base_client.get_vmt()).frame_stage_notify
        );
        InitVmtHook!(
            ptr_hooks,
            PaintTraverseHook,
            &(*interfaces.panel.get_vmt()).paint_traverse
        );
        InitVmtHook!(
            ptr_hooks,
            PaintHook,
            &(*interfaces.engine_vgui.get_vmt()).paint
        );
        InitVmtHook!(
            ptr_hooks,
            CreateMoveHook,
            &(*interfaces.client_mode.get_vmt()).create_move
        );
        InitVmtHook!(
            ptr_hooks,
            RunCommandHook,
            &(*interfaces.prediction.get_vmt()).run_command
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
            let exprted_fn: *const u8 = transmute(dlsym(
                handle,
                CString::new("SDL_GL_SwapWindow").unwrap().as_ptr(),
            ));
            let jump_dist = (exprted_fn.byte_add(6) as *const i32).read() as usize;
            let swap_window_ptr = exprted_fn.byte_add(6 + jump_dist + 4);

            InitVmtHook!(ptr_hooks, SwapWindowHook, transmute(swap_window_ptr));

            let exprted_fn: *const u8 = transmute(dlsym(
                handle,
                CString::new("SDL_PollEvent").unwrap().as_ptr(),
            ));
            let jump_dist = (exprted_fn.byte_add(6) as *const i32).read() as usize;
            let poll_event_ptr = exprted_fn.byte_add(6 + jump_dist + 4);
            InitVmtHook!(ptr_hooks, PollEventHook, transmute(poll_event_ptr));
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
