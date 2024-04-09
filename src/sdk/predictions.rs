
use crate::{cfn, impl_has_vmt, math::angles::Angles};

use super::{entity::player::Player, game_movement::CMoveData, user_cmd::UserCmd};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct VMTMoveHelper{
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct MoveHelper {
    vmt: *mut VMTMoveHelper,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTPrediction {
    _pad1: [u32; 13],
    pub get_local_view_angles: cfn!((), &mut Prediction, &mut Angles),
    pub set_local_view_angles: cfn!((), &Prediction, &Angles),
    _pad2: [u32; 3],
    pub run_command: cfn!((), &Prediction, &Player, &UserCmd, &MoveHelper),
    pub setup_move: cfn!(
        (),
        &Prediction,
        &Player,
        &UserCmd,
        &MoveHelper,
        &mut CMoveData
    ),
    pub finish_move: cfn!((), &Prediction, &Player, &UserCmd),
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Prediction {
    pub vmt: *const VMTPrediction,
    last_ground: isize,
    in_prediction: bool,
    first_time_predicted: bool,
    old_cl_predict_value: bool,
    engine_paused: bool,
    previous_start_frame: isize,
    commands_predicted: isize,
    server_commands_acknowledged: isize,
    previous_ack_had_errors: isize,
    incoming_packet_number: isize,
    ideal_pitch: f32,
}
unsafe impl Send for Prediction {}

impl_has_vmt!(Prediction, VMTPrediction);
