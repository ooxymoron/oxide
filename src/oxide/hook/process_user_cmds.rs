use std::mem::transmute;

use libc::pause;

use crate::{
    call_original, cfn, get_cheat, log, o,
    oxide::cheat::visual::Visuals,
    sdk::{
        entity::{player::Player, weapon::Weapon},
        fire_bullets_info::FireBulletsInfo,
        user_cmd::{ButtonFlags, UserCmd},
    },
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
    if cmds.buttons.get(ButtonFlags::InAttack) {
        let time = (o!().util.plat_float_time)() as f32;
        let seed = unsafe { transmute::<_, i32>(time * 1000.0) } & 0xFF;
        log!("server seed:    {}\tnum: {}\ttime: {}\tnum_count: {}", seed, cmds.command_number,time,num_cmds);
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
