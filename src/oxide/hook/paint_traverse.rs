use std::ffi::CStr;

use crate::{
    define_hook,
    sdk::interfaces::panel::{Panel, VPanel},
    setting, vmt_call,
};

fn subhooks(hook: &mut PaintTraverseHook) {
    hook.before = Some(|panel, vpanel, _, _| {
        let panel_name = unsafe { CStr::from_ptr(vmt_call!(panel, get_name, vpanel)) };
        match panel_name.to_str() {
            Ok("HudScope") => {
                if setting!(visual, remove_scope) {
                    Some(())
                }else {
                    None
                }
            }
            _ => None,
        }
    });
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
