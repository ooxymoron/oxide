use std::{ffi::{CStr, CString}, mem::transmute};

use crate::{c_str_to_str, call_original, cfn, sdk::game_event::GameEvent, util::debug::{print_bytes, print_module_addres_offset}, vmt_call};


pub const NAME: &str = "FireEvent";

pub type FireEvent = cfn!(bool, *const u8, &GameEvent, bool);

pub extern "C" fn load_whitelist_hook(event_manager: *const u8, event: &GameEvent, no_boradcast: bool) -> bool {
    //let name = c_str_to_str!(vmt_call!(event,get_name));
    call_original!(NAME, FireEvent, event_manager, event,no_boradcast)
}
