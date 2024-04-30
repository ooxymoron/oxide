use std::{
    alloc::{alloc, Layout},
    mem::ManuallyDrop,
};

use libc::c_void;
use sdl2_sys::SDL_Window;

use crate::{d, define_hook, draw::Draw, init_global, DRAW};

fn hook(window: *mut SDL_Window,org:SwapWindowHook::RawFn) {
    unsafe {
        if DRAW.is_none() {
            init_global!(DRAW,Draw::init(window).unwrap(),Draw);
            d!().load_components();
        }
        d!().run(window);
    }
    (org)(window)
}

define_hook!(
    SwapWindowHook,
    "SwapWindow",
    hook,
    (),
    (),
    window,
    *mut SDL_Window
);
