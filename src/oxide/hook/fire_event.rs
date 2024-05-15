use std::ffi::CStr;

use crate::oxide::cheat::crit_manipulation::CritManipulation;
use crate::oxide::cheat::visual::Visuals;
use crate::sdk::{event_manager::GameEventManager, game_event::GameEvent};

use crate::{define_hook, get_cheat, vmt_call};
fn hook(event_manager: &GameEventManager, event: &GameEvent, org: FireEventHook::RawFn) -> bool {
    let name = unsafe { CStr::from_ptr(vmt_call!(event, get_name)) }
        .to_str()
        .unwrap();
    match name {
        "player_hurt" => get_cheat!(CritManipulation).player_hurt(&event),
        "spec_target_updated" => {
            get_cheat!(Visuals).update_spectators().unwrap();
        }
        _ => {}
    }
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
