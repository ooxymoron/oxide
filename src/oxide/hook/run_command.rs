use core::prelude;

use crate::{
    define_hook,
    sdk::{
        entity::player::Player,
        interfaces::predictions::{MoveHelper, Prediction},
        user_cmd::UserCmd,
    },
};

fn hook(
    prediction: &Prediction,
    player: &Player,
    user_cmd: &UserCmd,
    move_helper: &'static MoveHelper,
    org: RunCommandHook::RawFn,
) {
    if o!().engine_prediction.move_helper.is_none() {
        o!().engine_prediction.move_helper = Some(move_helper);
    }
    (org)(prediction, player, user_cmd, move_helper)
}

define_hook!(
    RunCommandHook,
    "RunCommand",
    hook,
    (),
    (),
    prediction,
    &Prediction,
    player,
    &Player,
    cmd,
    &UserCmd,
    move_helper,
    &'static MoveHelper
);
