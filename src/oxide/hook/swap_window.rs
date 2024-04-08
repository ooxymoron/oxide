use std::{
    alloc::{alloc, Layout},
    mem::ManuallyDrop,
};

use libc::c_void;

use crate::{d, define_hook, draw::Draw, init_global, DRAW};

fn subhooks(hook: &mut SwapWindowHook) {
    hook.before = Some(|window| unsafe {
        if DRAW.is_none() {
            init_global!(DRAW,Draw::init(window)?,Draw);
        }
        d!().run(window);
        Ok(true)
    });
    hook.after = Some(|_, _| Ok(()));
}

define_hook!(
    SwapWindowHook,
    "SwapWindow",
    (),
    (),
    subhooks,
    window,
    *mut sdl2_sys::SDL_Window
);
