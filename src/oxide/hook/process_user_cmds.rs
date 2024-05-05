use crate::{
    call_original, cfn,
    sdk::{entity::player::Player, user_cmd::UserCmd},
};

pub const NAME: &str = "ProcessUserCmds";

pub type ProcessUserCmds = cfn!((), &mut Player, &UserCmd, i32, i32, i32, bool);

pub extern "C" fn hook(
    player: &mut Player,
    cmds: &UserCmd,
    num_cmds: i32,
    total_cmds: i32,
    dropped_packets: i32,
    paused: bool,
) {
    call_original!(
        NAME,
        ProcessUserCmds,
        player,
        cmds,
        num_cmds,
        total_cmds,
        dropped_packets,
        paused
    );
}
