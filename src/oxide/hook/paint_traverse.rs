use std::ffi::CStr;

use crate::{
    define_hook,
    sdk::interfaces::panel::{Panel, VPanel},
    setting, vmt_call,
};

fn hook(panel: &Panel, vpanel: VPanel, force_paint: bool, allow_force: bool, org: PaintTraverseHook::RawFn){
    let panel_name = unsafe { CStr::from_ptr(vmt_call!(panel, get_name, vpanel)) };
    let mut draw = true;
    match panel_name.to_str() {
        Ok("HudScope") => {
            draw = !*setting!(visual, remove_scope)
        }
        _ => (),
    }
    if draw {
        (org)(panel,vpanel,force_paint,allow_force)
    }
}

define_hook!(
    PaintTraverseHook,
    "PaintTraverse",
    hook,
    (),
    (),
    panel,
    &Panel,
    vpanel,
    VPanel,
    force_paint,
    bool,
    allow_force,
    bool
);
