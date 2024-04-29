use std::ffi::c_char;

use crate::{
    c_str_to_str, call_original, cfn,
    draw::colors::{LIGHT_BLUE, LIGHT_RED},
    interface,
    sdk::{effect_data::EffectData, entity::player::Player, networkable::ClassId},
};

pub const NAME: &str = "DispatchEffect";

pub type DispatchEffect = cfn!(bool, *const c_char, &EffectData);

pub extern "C" fn dispatch_effect(name: *const c_char, effect: &EffectData) -> bool {
    //FIXME: GETTING ENT HANDLE IS BROKEN
    let effect_name = c_str_to_str!(name);
    match effect_name {
        "Impact" => {
            let mut color = LIGHT_BLUE;
            let mut duration = 0.5;
            if let Some(ent) = effect.entity.resolve() {
                match ent.as_networkable().get_client_class().class_id {
                    ClassId::CTFPlayer
                    | ClassId::CObjectSentrygun
                    | ClassId::CObjectDispenser
                    | ClassId::CObjectTeleporter => {
                        if Player::get_local().is_ok() {
                            color = LIGHT_RED;
                            duration = 2.0;
                        }
                    }
                    _ => {}
                }
            }

            interface!(debug_overlay).rect(&effect.origin, 4.0, color, 50, duration);
        }
        _ => {}
    }
    call_original!(NAME, DispatchEffect, name, effect)
}
