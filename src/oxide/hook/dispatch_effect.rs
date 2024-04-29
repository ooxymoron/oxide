use std::ffi::c_char;

use crate::{c_str_to_str, call_original, cfn, draw::colors::LIGHT_BLUE, interface, log, sdk::effect_data::EffectData, vmt_call};


pub const NAME: &str = "DispatchEffect";

pub type DispatchEffect = cfn!(bool, *const c_char, &EffectData);

pub extern "C" fn dispatch_effect(
    name: *const c_char,
    effect: &EffectData,
) -> bool {
    let effect_name = c_str_to_str!(name);
    match effect_name {
        "Impact" => {

            interface!(debug_overlay).add_box_overlay(&effect.origin, 4.0, LIGHT_BLUE, 200, 2.0);
        }
        _ => {}
    }
    call_original!(NAME, DispatchEffect, name, effect)
}
