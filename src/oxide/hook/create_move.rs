use std::{intrinsics::breakpoint, mem::MaybeUninit, ptr::null};

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

struct EnginePrediction {
    old_frametime: f32,
    old_time: f32,
    old_in_prediction: bool,
    old_first_time_predicted: bool,
    old_command: *const UserCmd,
}

impl EnginePrediction {
    pub fn new(p_local: &mut Player) -> EnginePrediction {
        let old_frametime = o!().global_vars().frametime;
        let old_time = o!().global_vars().time;
        let old_in_prediction = interface!(prediction).in_prediction;
        let old_first_time_predicted = interface!(prediction).first_time_predicted;
        let old_command = p_local.current_command;

        EnginePrediction{old_command, old_first_time_predicted, old_frametime, old_in_prediction, old_time}
    }

    pub fn start(&mut self, p_local: &mut Player, cmd: &mut UserCmd) {
        if let Some(move_helper) = o!().move_helper{
            //unsafe { breakpoint() };

            o!().global_vars_mut().frametime = o!().global_vars().interval_per_tick;
            o!().global_vars_mut().time = (p_local.tick_base as f32) * o!().global_vars().interval_per_tick;
            interface!(prediction).first_time_predicted = false;
            interface!(prediction).in_prediction = true;
            p_local.current_command = cmd;
            
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
                &mut move_data
            );
            vmt_call!(
                interface!(prediction),
                finish_move,
                p_local,
                cmd,
                &mut move_data
            );
    
            vmt_call!(interface!(game_movement), finish_prediction, p_local);
    
            interface!(prediction).first_time_predicted = self.old_first_time_predicted;
            interface!(prediction).in_prediction = self.old_in_prediction;
        }
    }

    pub fn restore(&self, p_local: &mut Player) {
        p_local.current_command = self.old_command;
        o!().global_vars_mut().frametime = self.old_frametime;
        o!().global_vars_mut().time = self.old_time;
    }
    
}


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
        
        let mut aimbot = o!().cheats.get::<Aimbot>(Aimbot::name());
        let mut engine_prediction = EnginePrediction::new(p_local);

        if setting!(aimbot, engine_prediction) {
            engine_prediction.start(p_local, cmd);
        }
        
        aimbot.create_move(cmd)?;

        if setting!(aimbot, engine_prediction) {
            engine_prediction.restore(p_local);
        }
        

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
