use crate::{
    define_hook,
    oxide::cheat::visual::Visuals,
    sdk::{
        interfaces::client_mode::ClientMode,
        view_setup::{self, ViewSetup},
    },
};

fn hook(client_move: &mut ClientMode, view_setup: &mut ViewSetup, org: OverrideViewHook::RawFn) {
    let mut visuals = o!().cheats.get::<Visuals>(Visuals::name());
    visuals.override_view(view_setup);
    o!().fov = view_setup.fov;
    (org)(client_move, view_setup)
}

define_hook!(
    OverrideViewHook,
    "OverrideView",
    hook,
    (),
    (),
    client_move,
    &mut ClientMode,
    view_setup,
    &mut ViewSetup
);
