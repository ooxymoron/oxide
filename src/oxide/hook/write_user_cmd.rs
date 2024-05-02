use crate::{call_original, cfn, sdk::user_cmd::UserCmd};

pub const NAME: &str = "WriteUserCmd";

pub type WriteUserCmd = cfn!((), *const u8, &UserCmd, &UserCmd);

pub extern "C" fn hook(buf: *const u8, from: &UserCmd, to: &UserCmd) {
    call_original!(NAME, WriteUserCmd, buf, from, to)
}
