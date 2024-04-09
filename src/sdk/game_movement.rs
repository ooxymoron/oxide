use crate::{
    cfn,
    math::{angles::Angles, vector::Vector3},
};

use super::{
    entity::player::Player,
    CBaseHandle, WithVmt,
};

pub type GameMovement = WithVmt<VMTGameMovement>;

type EntityHandle = CBaseHandle;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CMoveData {
    first_run_of_functions: bool,
    game_code_moved_player: bool,
    player_handle: EntityHandle,
    impulse_command: isize,
    view_angles: Angles,
    abs_view_angles: Angles,
    buttons: isize,
    old_buttons: isize,
    forward_bove: f32,
    old_forward_bove: f32,
    sidemove: f32,
    up_move: f32,
    max_speed: f32,
    client_max_speed: f32,
    velocity: Vector3,
    angles: Angles,
    old_angles: Angles,
    step_height: f32,
    wish_vel: Vector3,
    jump_vel: Vector3,
    constraint_center: Vector3,
    constraint_radius: f32,
    constraint_width: f32,
    constraint_speed_factor: f32,
    abs_origin: Vector3,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTGameMovement {
    _pad1: [u32; 2],
    pub process_movement: cfn!((), &GameMovement, &Player, &CMoveData),
    pub start_prediction: cfn!((), &GameMovement, &Player),
    pub finish_prediction: cfn!((), &GameMovement, &Player),
}
