use std::{intrinsics::breakpoint, mem::transmute};

use crate::{
    call_original, cfn,
    math::{angles::Angles, vector::Vector3},
    o,
};

pub const NAME: &str = "BaseInterpolatePart1Hook";

pub type BaseInterpolatePart1HookType = cfn!(isize, f32, Vector3, Angles, Vector3, isize);

pub extern "C" fn BaseInterpolatePart1Hook(
    curr_time: f32,
    old_origin: Vector3,
    old_angle: Angles,
    old_vel: Vector3,
    no_more_changes: isize,
) -> isize {
    let hook = o!().hooks.detour_hooks.get_mut(NAME).unwrap();
    hook.unpatch();

    let res = unsafe {
        transmute::<_, BaseInterpolatePart1HookType>(hook.target)(
            curr_time,
            old_origin,
            old_angle,
            old_vel,
            no_more_changes,
        )
    };
    hook.patch();
    return res;
    //call_original!(NAME);
}
