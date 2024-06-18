use crate::{
    define_hook, get_cheat,
    oxide::cheat::{
        aimbot::Aimbot, crit_manipulation::CritManipulation, movement::Movement,
        spread_reduction::SpreadReduction, triggerbot::Triggerbot,
    },
    sdk::{entity::player::Player, interfaces::client_mode::ClientMode, user_cmd::UserCmd},
    setting, vmt_call,
};

fn hook(
    client_mode: &mut ClientMode,
    input_sample_time: f32,
    cmd: &mut UserCmd,
    org: CreateMoveHook::RawFn,
) -> bool {
    (org)(client_mode, input_sample_time, cmd);
    if cmd.command_number == 0 {
        return false;
    }
    let p_local = Player::get_local().unwrap();

    if !vmt_call!(p_local.as_ent(), is_alive) {
        return false;
    }

    let org_cmd = cmd.clone();

    let mut movement = get_cheat!(Movement);
    movement.create_move(cmd).unwrap();

    if o!().engine_prediction.move_helper.is_some() {
        if o!().engine_prediction.data.is_some() {
            o!().engine_prediction.finish().unwrap();
        }
        o!().engine_prediction.init(p_local, cmd).unwrap();
        o!().engine_prediction.step().unwrap();
    }

    let mut aimbot = get_cheat!(Aimbot);
    let target = aimbot.create_move(cmd).unwrap();

    if o!().engine_prediction.move_helper.is_some() {
        o!().engine_prediction.finish().unwrap();
    }
    get_cheat!(Triggerbot).create_move(cmd).unwrap();
    get_cheat!(CritManipulation).create_move(cmd);
    let mut spread_reduction = get_cheat!(SpreadReduction);
    spread_reduction.create_move(cmd, target);

    if *setting!(aimbot, silent) {
        remove_punch(cmd);
    }
    movement.create_move_after(cmd, &org_cmd);
    !*setting!(aimbot, silent)
}

define_hook!(
    CreateMoveHook,
    "CreateMove",
    hook,
    bool,
    true,
    client_mode,
    &mut ClientMode,
    input_sample_time,
    f32,
    cmd,
    &mut UserCmd
);

pub fn remove_punch(cmd: &mut UserCmd) {
    let p_local = Player::get_local().unwrap();
    cmd.viewangles -= *p_local.get_punch_angle();
}
