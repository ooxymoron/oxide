use std::ptr::null;

use crate::{call_original, cfn, setting};

pub const NAME: &str = "LoadWhitelistHook";

pub type LoadWhitelist = cfn!(*const u8, *const u8);

pub extern "C" fn load_whitelist_hook(table: *const u8) -> *const u8 {
    if setting!(visual, pure_bypass) {
        return null();
    }
    call_original!(NAME, LoadWhitelist, table)
}
