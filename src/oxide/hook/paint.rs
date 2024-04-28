use crate::{define_hook, sdk::interfaces::engine_vgui::EngineVgui};

fn subhooks(hook: &mut PaintHook) {
    hook.after = Some(|_, _, _| {
        o!().paint.paint().unwrap();
    });
}
define_hook!(
    PaintHook,
    "Paint",
    (),
    (),
    subhooks,
    engine_vgui,
    &EngineVgui,
    mode,
    isize
);
