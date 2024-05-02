use crate::{call_original, cfn, get_cheat, oxide::cheat::spread_reduction::SpreadReduction, sdk::user_cmd::UserCmd};

pub const NAME: &str = "WriteUserCmd";

pub type WriteUserCmd = cfn!((), *const u8, &UserCmd, &UserCmd);

pub extern "C" fn hook(buf: *const u8, from: &UserCmd, to: &UserCmd) {
    get_cheat!(SpreadReduction).write_user_cmd(to);
    call_original!(NAME, WriteUserCmd, buf, from, to)
}
