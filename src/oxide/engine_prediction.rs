use std::mem::{transmute, MaybeUninit};

use crate::{
    error::{OxideError, OxideResult},
    interface, o,
    sdk::{
        entity::player::Player, game_movement::MoveData, predictions::MoveHelper, user_cmd::UserCmd,
    },
    vmt_call,
};
#[derive(Debug)]
pub struct PredictionData {
    old_curtime: f32,
    old_frametime: f32,
    old_tick_count: i32,
    old_in_predicion: bool,
    old_first_time_predicted: bool,
    player: *mut Player,
    cmd: *mut UserCmd,
    move_data: MoveData,
}

impl PredictionData {
    pub fn new(player: *mut Player, cmd: *mut UserCmd) -> Self {
        PredictionData {
            old_curtime: o!().global_vars.curtime,
            old_frametime: o!().global_vars.frametime,
            old_tick_count: o!().global_vars.tick_count,
            old_in_predicion: interface!(prediction).in_prediction,
            old_first_time_predicted: interface!(prediction).first_time_predicted,
            player,
            cmd,
            move_data: unsafe { MaybeUninit::zeroed().assume_init() },
        }
    }
    pub fn restore(&self) {
        interface!(prediction).in_prediction = self.old_in_predicion;
        interface!(prediction).first_time_predicted = self.old_first_time_predicted;
        o!().global_vars.curtime = self.old_curtime;
        o!().global_vars.frametime = self.old_frametime;
        o!().global_vars.tick_count = self.old_tick_count;
    }
    pub fn player(&self) -> &'static mut Player {
        unsafe { transmute(self.player) }
    }
    pub fn cmd(&self) -> &'static mut UserCmd {
        unsafe { transmute(self.cmd) }
    }
}

#[derive(Debug)]
pub struct EnginePredicion {
    pub move_helper: Option<&'static MoveHelper>,
    pub data: Option<PredictionData>,
}

impl EnginePredicion {
    pub fn new() -> Self {
        Self {
            move_helper: None,
            data: None,
        }
    }
    pub fn init(&mut self, player: &mut Player, cmd: &mut UserCmd) -> OxideResult<()> {
        if !vmt_call!(player.as_ent(), is_alive) {
            return Err(OxideError::new("player dead"));
        }
        if self.data.is_some() {
            return Err(OxideError::new("already predicting"));
        }
        let Some(move_helper) = self.move_helper else {
            return Err(OxideError::new("no move helper"))
        };

        self.data = Some(PredictionData::new(player, cmd));

        let current_command = player.get_current_command();
        *current_command = cmd;

        let tick_base = *player.get_tick_base();

        o!().global_vars.curtime = tick_base as f32 * o!().global_vars.interval_per_tick;
        o!().global_vars.frametime = o!().global_vars.interval_per_tick;
        o!().global_vars.tick_count = tick_base;
        interface!(prediction).in_prediction = true;
        interface!(prediction).first_time_predicted = false;

        vmt_call!(interface!(game_movement), start_prediction, player);
        vmt_call!(
            interface!(prediction),
            set_local_view_angles,
            &cmd.viewangles
        );

        vmt_call!(
            interface!(prediction),
            setup_move,
            player,
            cmd,
            move_helper,
            &mut self.data.as_mut().unwrap().move_data
        );
        Ok(())
    }
    pub fn step(&mut self) -> OxideResult<()> {
        let Some(data) = &mut self.data else {
            return Err(OxideError::new("not predicting"));
        };

        vmt_call!(
            interface!(game_movement),
            process_movement,
            data.player(),
            &data.move_data
        );
        vmt_call!(
            interface!(prediction),
            finish_move,
            data.player(),
            data.cmd(),
            &data.move_data
        );

        Ok(())
    }
    pub fn finish(&mut self) -> OxideResult<()> {
        let Some(data) = &mut self.data else {
            return Err(OxideError::new("not predicting"));
        };
        data.restore();
        //*data.player().get_current_command() = unsafe { transmute(0i64) };
        vmt_call!(interface!(game_movement), finish_prediction, data.player());
        self.data = None;
        Ok(())
    }
}
