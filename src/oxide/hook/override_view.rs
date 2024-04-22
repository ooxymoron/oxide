use crate::{
    define_hook,
    oxide::cheat::visual::Visuals,
    sdk::{client_mode::ClientMode, view_setup::ViewSetup},
};

fn subhooks(hook: &mut OverrideViewHook) {
    hook.before = Some(|_, view_setup| {
        let mut visuals = o!().cheats.get::<Visuals>(Visuals::name());
        visuals.override_view(view_setup);
        o!().fov = view_setup.fov;
        Ok(None)
    });
    hook.after = Some(|_, _, _| Ok(()));
}

define_hook!(
    OverrideViewHook,
    "OverrideView",
    (),
    (),
    subhooks,
    client_move,
    &mut ClientMode,
    view_setup,
    &mut ViewSetup
);
