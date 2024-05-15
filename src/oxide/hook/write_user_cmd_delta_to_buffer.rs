use std::mem::{transmute, MaybeUninit};

use crate::{
    cfn,
    oxide::hook::{get_user_cmd, validate_user_cmd, write_user_cmd},
    sdk::{input::Input, user_cmd::UserCmd},
};

pub const NAME: &str = "WriteUserCmdDeltaToBufer";

pub type WriteUserCmdDeltaToBuffer = cfn!(bool, &Input, *const u8, i32, i32);

pub extern "C" fn hook(input: &Input, buf: *const u8, from: i32, to: i32) -> bool {
    let from_cmd = if from != -1 {
        get_user_cmd::hook(input, from)
    } else {
        unsafe { transmute(MaybeUninit::<UserCmd>::zeroed().assume_init_ref()) }
    };
    validate_user_cmd::hook(input, from_cmd, from);

    let to_cmd = get_user_cmd::hook(input, to);
    validate_user_cmd::hook(input, to_cmd, to);

    write_user_cmd::hook(buf, to_cmd, from_cmd);
    true
}
