use crate::{cfn, impl_has_vmt, math::angles::Angles};

use super::{entity::player::Player, game_movement::MoveData, user_cmd::UserCmd};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct VMTMoveHelper {}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct MoveHelper {
    vmt: *mut VMTMoveHelper,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTPrediction {
    _pad1: [usize; 13],
    pub get_local_view_angles: cfn!((), &mut Prediction, &mut Angles),
    pub set_local_view_angles: cfn!((), &Prediction, &Angles),
    _pad2: [usize; 3],
    pub run_command: cfn!((), &Prediction, &Player, &UserCmd, &MoveHelper),
    pub setup_move: cfn!(
        (),
        &Prediction,
        &Player,
        &UserCmd,
        &MoveHelper,
        &mut MoveData
    ),
    pub finish_move: cfn!((), &Prediction, &Player, &UserCmd, &MoveData),
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Prediction {
    pub vmt: *const VMTPrediction,
    pub last_ground: isize,
    pub in_prediction: bool,
    pub first_time_predicted: bool,
    pub old_cl_predict_value: bool,
    pub engine_paused: bool,
    pub previous_start_frame: isize,
    pub commands_predicted: isize,
    pub server_commands_acknowledged: isize,
    pub previous_ack_had_errors: isize,
    pub incoming_packet_number: isize,
    pub ideal_pitch: f32,
}
unsafe impl Send for Prediction {}

impl_has_vmt!(Prediction, VMTPrediction);
