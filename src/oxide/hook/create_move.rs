use crate::{
    define_hook, draw::colors::WHITE, get_cheat, interface, oxide::cheat::{aimbot::Aimbot, movement::Movement, spread_reduction::SpreadReduction}, sdk::{entity::player::Player, interfaces::{client_mode::ClientMode, engine_trace::{trace, CONTENTS_GRATE, MASK_SHOT}}, user_cmd::UserCmd}, setting, vmt_call
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

    let mut movement = o!().cheats.get::<Movement>(Movement::name());
    movement.create_move(cmd).unwrap();

    if o!().engine_prediction.move_helper.is_some() {
        if o!().engine_prediction.data.is_some() {
            o!().engine_prediction.finish().unwrap();
        }
        o!().engine_prediction.init(p_local, cmd).unwrap();
        o!().engine_prediction.step().unwrap();
    }

    let mut aimbot = o!().cheats.get::<Aimbot>(Aimbot::name());
    aimbot.create_move(cmd).unwrap();

    if o!().engine_prediction.move_helper.is_some() {
        o!().engine_prediction.finish().unwrap();
    }
    let mut spread_reduction = get_cheat!(SpreadReduction);

    //remove_punch(p_local);
    spread_reduction.create_move(cmd);
    movement.correct_movement(cmd, &org_cmd);
    if setting!(visual, impacts) || setting!(visual, tracers) {
        let dir = cmd.viewangles.to_vectors().forward * 1000.0;
        let src = vmt_call!(p_local.as_ent(),eye_position);
        let trace = trace(src, dir + src, MASK_SHOT | CONTENTS_GRATE);
        let color = WHITE;
        let alpha = 50;
        let time = 4.0;
        if setting!(visual, impacts) {
            interface!(debug_overlay).rect(&trace.endpos, 4.0, color, alpha, time);
        }
        if setting!(visual, tracers) {
            interface!(debug_overlay).line(
                &trace.startpos,
                &trace.endpos.clone(),
                color,
                alpha,
                time,
            );
        }
    }

    if let Some(calculation_start) = get_cheat!(SpreadReduction).calculation_start {
        let time = (o!().util.plat_float_time)() as f32;
        spread_reduction.calculation_delay = time - calculation_start;
        spread_reduction.calculation_start = None;
    }
    !setting!(aimbot, silent)
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

pub fn remove_punch(p_local: &Player) {
    let mut my_angles = vmt_call!(p_local.as_ent(), get_abs_angles).clone();
    let punch_angle = p_local.get_punch_angle();
    my_angles.pitch += punch_angle.pitch;
    my_angles.yaw += punch_angle.yaw;
    my_angles.roll += punch_angle.roll;
}
