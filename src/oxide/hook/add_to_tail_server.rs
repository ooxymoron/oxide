use crate::{
    call_original, cfn, get_cheat, oxide::cheat::spread_reduction::{seed_prediction::State, SpreadReduction},
    sdk::user_cmd::ServerUserCmd,
};

pub const NAME: &str = "AddToTail";

pub type AddToTail = cfn!(i32, *const u8, i32, &mut ServerUserCmd);

pub extern "C" fn hook(ctx: *const u8, something: i32, cmd: &mut ServerUserCmd) -> i32 {
    if let State::SYNCED {last_seed,..} = get_cheat!(SpreadReduction).state {
        if let Some(last_seed) = last_seed {
            cmd.server_seed = last_seed;
        }
    }
    call_original!(NAME, AddToTail, ctx, something, cmd)
}
