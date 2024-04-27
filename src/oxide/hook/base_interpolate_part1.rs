
use crate::{
    call_original, cfn,
    math::{angles::Angles, vector::Vector3},
};

pub const NAME: &str = "BaseInterpolatePart1Hook";

pub type BaseInterpolatePart1HookType = cfn!(isize, f32, Vector3, Angles, Vector3, isize);

pub extern "C" fn base_interpolate_part1_hook(
    curr_time: f32,
    old_origin: Vector3,
    old_angle: Angles,
    old_vel: Vector3,
    no_more_changes: isize,
) -> isize {
    call_original!(
        NAME,
        BaseInterpolatePart1HookType,
        curr_time,
        old_origin,
        old_angle,
        old_vel,
        no_more_changes
    )
}
