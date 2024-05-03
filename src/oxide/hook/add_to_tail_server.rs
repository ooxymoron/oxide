
use crate::{
    call_original, cfn, oxide::hook::process_user_cmds::LAST_SERVER_SEED, sdk::user_cmd::UserCmd,
};

pub const NAME: &str = "AddToTail";

pub type AddToTail = cfn!(i32, *const u8, i32, &mut UserCmd);

pub extern "C" fn hook(ctx: *const u8, something: i32, cmd: &mut UserCmd) -> i32 {
    unsafe {
        cmd.seed = LAST_SERVER_SEED;
    }
    call_original!(NAME, AddToTail, ctx, something, cmd)
}
