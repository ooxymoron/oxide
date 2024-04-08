use std::ffi::CStr;

use crate::{
    vmt_call, define_hook, setting,
    sdk::panel::{Panel, VPanel},
};

fn subhooks(hook: &mut PaintTraverseHook) {
    hook.before = Some(|panel, vpanel, _, _| {
        let panel_name = unsafe { CStr::from_ptr(vmt_call!(panel, get_name, vpanel)) };
        Ok(match panel_name.to_str() {
            Ok("HudScope") => !setting!(visual,remove_scope),
            _ => true,
        })
    });
    hook.after = Some(|_, _, _, _, _| Ok(()));
}

define_hook!(
    PaintTraverseHook,
    "PaintTraverse",
    (),
    (),
    subhooks,
    panel,
    &Panel,
    vpanel,
    VPanel,
    force_paint,
    bool,
    allow_force,
    bool
);
