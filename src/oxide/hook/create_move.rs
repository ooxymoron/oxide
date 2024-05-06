use crate::{
    define_hook,
    draw::colors::WHITE,
    get_cheat, interface,
    math::angles::Angles,
    oxide::cheat::{aimbot::Aimbot, movement::Movement, spread_reduction::SpreadReduction},
    sdk::{
        entity::player::Player,
        interfaces::{
            client_mode::ClientMode,
            engine_trace::{trace, CONTENTS_GRATE, MASK_SHOT},
        },
        user_cmd::{ButtonFlags, UserCmd},
    },
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
    let target = aimbot.create_move(cmd).unwrap();

    if o!().engine_prediction.move_helper.is_some() {
        o!().engine_prediction.finish().unwrap();
    }
    let mut spread_reduction = get_cheat!(SpreadReduction);
    spread_reduction.create_move(cmd, target);

    //todo move this to visuals
    if (setting!(visual, impacts) || setting!(visual, tracers))
        && cmd.buttons.get(ButtonFlags::InAttack)
        && p_local.can_attack()
    {
        let weapon = vmt_call!(p_local.as_ent(),get_weapon);
        let range = weapon.get_info().weapon_data[weapon.get_mode()].range;
        let dir = cmd.viewangles.to_vectors().forward * range;
        let src = vmt_call!(p_local.as_ent(), eye_position);
        let trace = trace(src, src + dir, MASK_SHOT | CONTENTS_GRATE);
        let color = WHITE;
        let alpha = 20;
        let time = 0.5;
        if setting!(visual, impacts) {
            interface!(debug_overlay).rect(&trace.endpos, 4.0, color, alpha, time);
            interface!(debug_overlay).triangle(&src, 4.0, color, alpha, time);
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
    remove_punch();
    movement.create_move_after(cmd, &org_cmd);
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

pub fn remove_punch() {
    let p_local = Player::get_local().unwrap();
    let punch_angle = p_local.get_punch_angle();
    *punch_angle = Angles::new(0.0, 0.0, 0.0);
}
