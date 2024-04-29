use crate::{
    c_str_to_str, call_original, cfn, draw::{colors::LIGHT_BLUE, event::Event}, interface, log, math::vector::Vector3, sdk::{
        entity::{player::Player, Entity},
        game_event::GameEvent,
    }, str_to_c_str, vmt_call
};

pub const NAME: &str = "FireEvent";

pub type FireEvent = cfn!(bool, *const u8, &GameEvent, bool);

pub extern "C" fn fire_event(
    event_manager: *const u8,
    event: &GameEvent,
    no_boradcast: bool,
) -> bool {
    let event_name = c_str_to_str!(vmt_call!(event, get_name));
    log!("{}",event_name);
    call_original!(NAME, FireEvent, event_manager, event, no_boradcast)
}
