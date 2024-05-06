use std::ffi::CStr;

use crate::sdk::{event_manager::GameEventManager, game_event::GameEvent};

use crate::{define_hook, vmt_call};
fn hook(event_manager: &GameEventManager, event: &GameEvent, org: FireEventHook::RawFn) -> bool {
    let name = unsafe { CStr::from_ptr(vmt_call!(event, get_name)) }
        .to_str()
        .unwrap();
    log!("{}", name);
    (org)(event_manager, event)
}

define_hook!(
    FireEventHook,
    "FireEvent",
    hook,
    bool,
    false,
    event_manager,
    &GameEventManager,
    event,
    &GameEvent
);
