use crate::{
    call_original, cfn, get_cheat, o,
    oxide::cheat::spread_reduction::{SpreadReduction, MIN_MANTISA},
    sdk::{
        entity::player::Player,
        user_cmd::{ButtonFlags, UserCmd},
    },
    spread_prediction_log,
};

pub const NAME: &str = "ProcessUserCmds";

pub type ProcessUserCmds = cfn!((), &mut Player, &UserCmd, i32, i32, i32, bool);
pub static mut LAST_SERVER_SEED: i32 = 0;

pub extern "C" fn hook(
    player: &mut Player,
    cmds: &UserCmd,
    num_cmds: i32,
    total_cmds: i32,
    dropped_packets: i32,
    paused: bool,
) {
    if cmds.buttons.get(ButtonFlags::InAttack) {
        let time = (o!().util.plat_float_time)() as f32 + 2f32.powi(MIN_MANTISA);
        unsafe {
            //LAST_SERVER_SEED = transmute::<_, i32>(time * 1000f32) & 0xFF;
        let error = get_cheat!(SpreadReduction).last_predicted_time - time;

        spread_prediction_log!(
            "server seed:    {}\tnum: {}\ttime: {}\terror: {}",
            LAST_SERVER_SEED,
            cmds.command_number,
            time,
            error
        );
        }
    }
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
