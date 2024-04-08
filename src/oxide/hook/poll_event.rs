use sdl2_sys::SDL_Event;

use crate::define_hook;
fn subhooks(hook:&mut PollEventHook) {
    hook.before = Some(|e|{
        o!().handle_event(e);
        Ok(true)
    });
    hook.after = Some(|_,_|{
        Ok(())

    });
}

define_hook!(
    PollEventHook,
    "PollEvent",
    isize,
    1,
    subhooks,
    event,
    &mut SDL_Event
);
