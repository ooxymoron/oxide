use crate::{
    c_str_to_str, call_original, cfn, log,
    sdk::{
        game_event::GameEvent,
    },
    vmt_call,
};

pub const NAME: &str = "FireEvent";

pub type FireEvent = cfn!(bool, *const u8, &GameEvent, bool);

pub extern "C" fn hook(
    event_manager: *const u8,
    event: &GameEvent,
    no_boradcast: bool,
) -> bool {
    let event_name = c_str_to_str!(vmt_call!(event, get_name));
    log!("{}", event_name);
    call_original!(NAME, FireEvent, event_manager, event, no_boradcast)
}
