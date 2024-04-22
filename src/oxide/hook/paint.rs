use crate::{define_hook, sdk::engine_vgui::EngineVgui};

fn subhooks(hook: &mut PaintHook) {
    hook.before = Some(|_, _| Ok(None));
    hook.after = Some(|_, _, _| {
        o!().paint.paint()?;
        Ok(())
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
