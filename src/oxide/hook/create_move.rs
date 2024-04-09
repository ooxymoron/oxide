use std::{intrinsics::breakpoint, mem::MaybeUninit};

use crate::{
    define_hook, interface,
    oxide::cheat::{aimbot::Aimbot, movement::Movement},
    sdk::{
        client_mode::ClientMode,
        entity::{player::Player, Entity},
        user_cmd::UserCmd,
    },
    setting, vmt_call,
};

fn subhooks(hook: &mut CreateMoveHook) {
    hook.before = Some(|_, _, _| Ok(true));
    hook.after = Some(|_, _, cmd, res| {
        if cmd.command_number == 0 {
            return Ok(());
        }
        let p_local = Entity::get_local()?;

        if !vmt_call!(p_local.as_ent(), is_alive) {
            return Ok(());
        }

        let org_cmd = cmd.clone();

        if setting!(visual, third_person) {
            p_local.force_taunt_cam = 1
        } else {
            p_local.force_taunt_cam = 0
        }

        let mut movement = o!().cheats.get::<Movement>(Movement::name());
        movement.create_move(cmd, &org_cmd)?;

        if let Some(move_helper) = o!().move_helper {
            unsafe { breakpoint() };
            vmt_call!(interface!(game_movement), start_prediction, p_local);
            vmt_call!(
                interface!(prediction),
                set_local_view_angles,
                &cmd.viewangles
            );

            let mut move_data = unsafe { MaybeUninit::zeroed().assume_init() };
            vmt_call!(
                interface!(prediction),
                setup_move,
                p_local,
                cmd,
                move_helper,
                &mut move_data
            );
            vmt_call!(
                interface!(game_movement),
                process_movement,
                p_local,
                &move_data
            );

            vmt_call!(interface!(game_movement), finish_prediction, p_local);
        }

        let mut aimbot = o!().cheats.get::<Aimbot>(Aimbot::name());
        aimbot.create_move(cmd)?;

        remove_punch(p_local);
        *res = !setting!(aimbot, silent);
        Ok(())
    });
}

define_hook!(
    CreateMoveHook,
    "CreateMove",
    bool,
    true,
    subhooks,
    client_mode,
    &mut ClientMode,
    input_sample_time,
    f32,
    cmd,
    &mut UserCmd
);

pub fn remove_punch(p_local: &Player) {
    let mut my_angles = vmt_call!(p_local.as_ent(), get_abs_angles).clone();
    my_angles.pitch += p_local.vec_punch_angle.pitch;
    my_angles.yaw += p_local.vec_punch_angle.yaw;
    my_angles.roll += p_local.vec_punch_angle.roll;
}
