use crate::{
    define_hook,
    sdk::{
        client_mode::ClientMode, condition::ConditionFlags, entity::Entity, view_setup::ViewSetup,
    },
    setting,
};

fn subhooks(hook: &mut OverrideViewHook) {
    hook.before = Some(|_, view_setup| {
        let Ok(p_local) = Entity::get_local() else { return Ok(true)};
        if !p_local.condition.get(ConditionFlags::Zoomed)
            || (p_local.condition.get(ConditionFlags::Zoomed) && setting!(visual, remove_zoom))
        {
            view_setup.fov = setting!(visual, fov)
        };
        o!().fov = Some(view_setup.fov);
        Ok(true)
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
