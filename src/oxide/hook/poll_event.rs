use sdl2_sys::SDL_Event;

use crate::define_hook;
fn hook(e: &mut SDL_Event, org: PollEventHook::RawFn) -> isize {
    o!().handle_event(e);
    (org)(e)
}

define_hook!(
    PollEventHook,
    "PollEvent",
    hook,
    isize,
    1,
    event,
    &mut SDL_Event
);
