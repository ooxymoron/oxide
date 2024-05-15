use crate::{
    call_original, cfn,
    sdk::{input::Input, user_cmd::UserCmd},
};

pub const NAME: &str = "ValidateUserCmd";

pub type ValidateUserCmd = cfn!((), &Input, &UserCmd, i32);

pub extern "C" fn hook(input: &Input, cmd: &UserCmd, sequence_number: i32) {
    call_original!(NAME, ValidateUserCmd, input, cmd, sequence_number)
}
