use crate::{
    define_hook,
    oxide::cheat::{aimbot::Aimbot, movement::Movement},
    sdk::{
        interfaces::client_mode::ClientMode,
        entity::{player::Player, Entity},
        user_cmd::UserCmd,
    },
    setting, vmt_call,
};

fn subhooks(hook: &mut CreateMoveHook) {
    hook.after = Some(|_, _, cmd, res| {
        if cmd.command_number == 0 {
            return;
        }
        let p_local = Entity::get_local().unwrap();

        if !vmt_call!(p_local.as_ent(), is_alive) {
            return;
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

        movement.correct_movement(cmd, &org_cmd);
        remove_punch(p_local);
        *res = !setting!(aimbot, silent);
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
    let punch_angle = p_local.get_punch_angle();
    my_angles.pitch += punch_angle.pitch;
    my_angles.yaw += punch_angle.yaw;
    my_angles.roll += punch_angle.roll;
}
