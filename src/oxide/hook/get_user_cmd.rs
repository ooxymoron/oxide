use std::{mem::transmute, slice};

use crate::{
    cfn,
    sdk::{
        input::{Input, MULTIPLAYER_BACKUP},
        user_cmd::UserCmd,
    },
};

pub const NAME: &str = "GetUserCmd";

pub type GetUserCmd = cfn!(&'static UserCmd, &Input, i32);

pub extern "C" fn hook(input: &Input, sequence_number: i32) -> &'static UserCmd {
    unsafe {
        transmute(
            &slice::from_raw_parts(input.commands, MULTIPLAYER_BACKUP)
                [sequence_number as usize % MULTIPLAYER_BACKUP],
        )
    }
}
