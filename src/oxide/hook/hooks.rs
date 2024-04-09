use std::{
    collections::HashMap,
    mem::{transmute, ManuallyDrop},
};

use libc::c_void;

use crate::{
    oxide::{hook::run_command::RunCommandHook, interfaces::Interfaces},
    util::get_handle,
};

use super::{
    create_move::CreateMoveHook, frame_stage_notify::FrameStageNotifyHook,
    override_view::OverrideViewHook, paint::PaintHook, paint_traverse::PaintTraverseHook,
    poll_event::PollEventHook, swap_window::SwapWindowHook, Hook,
};

static SWAPWINDOW_OFFSET: usize = 0xFD648;
static POLLEVENT_OFFSET: usize = 0xFCF64;

#[derive(Debug)]

pub struct Hooks(HashMap<String, Box<dyn Hook + 'static>>);

macro_rules! InitHook {
    ($hooks:expr,$HookClass:ident,$val:expr) => {
        $hooks.insert(
            $HookClass::name(),
            Box::new($HookClass::init($val)) as Box<dyn Hook>,
        );
    };
}

impl Hooks {
    pub fn init(interfaces: &Interfaces) -> Hooks {
        let mut hooks = HashMap::new();

        InitHook!(
            hooks,
            OverrideViewHook,
            &(*interfaces.client_mode.get_vmt()).override_view
        );
        InitHook!(
            hooks,
            FrameStageNotifyHook,
            &(*interfaces.base_client.get_vmt()).frame_stage_notify
        );
        InitHook!(
            hooks,
            PaintTraverseHook,
            &(*interfaces.panel.get_vmt()).paint_traverse
        );
        InitHook!(hooks, PaintHook, &(*interfaces.engine_vgui.get_vmt()).paint);
        InitHook!(
            hooks,
            CreateMoveHook,
            &(*interfaces.client_mode.get_vmt()).create_move
        );
        InitHook!(
            hooks,
            RunCommandHook,
            &(*interfaces.prediction.get_vmt()).run_command
        );

        let sdl_handle = get_handle("./bin/libSDL2-2.0.so.0").unwrap() as *const _
            as *const *const *const c_void;

        unsafe {
            InitHook!(
                hooks,
                SwapWindowHook,
                transmute((*sdl_handle) as usize + SWAPWINDOW_OFFSET)
            );
            InitHook!(
                hooks,
                PollEventHook,
                transmute((*sdl_handle) as usize + POLLEVENT_OFFSET)
            );
        }

        Hooks(hooks)
    }
    pub fn get<T>(&mut self, name: String) -> ManuallyDrop<&mut Box<T>> {
        unsafe { ManuallyDrop::new(transmute(self.0.get_mut(&name).unwrap())) }
    }
    pub fn restore(&mut self) {
        for (_, hook) in &mut self.0 {
            hook.restore()
        }
    }
}
