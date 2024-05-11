use crate::{
    define_hook, get_cheat, oxide::cheat::visual::Visuals, sdk::{interfaces::client_mode::ClientMode, view_setup::ViewSetup}
};

fn hook(client_move: &mut ClientMode, view_setup: &mut ViewSetup, org: PreRenderHook::RawFn) {
    let mut visuals = get_cheat!(Visuals);
    visuals.pre_render(view_setup);
    o!().fov = view_setup.fov;
    (org)(client_move, view_setup)
}

define_hook!(
    PreRenderHook,
    "PreRender",
    hook,
    (),
    (),
    client_mode,
    &mut ClientMode,
    view_setup,
    &mut ViewSetup
);
