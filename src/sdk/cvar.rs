use std::{
    ffi::{CStr, CString},
    mem::transmute,
};

use crate::{
    vmt_call,
    error::{OxideError, OxideResult},
    interface,
};

use self::convar::ConVar;

use super::*;

pub type CVar = WithVmt<VMTCVar>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTCVar {
    _pad: [u32;  12],
    find_var: cfn!(*mut ConVar, &CVar, *const i8),
    pub find_var_const: cfn!(*const ConVar, &CVar, *const i8),
    pub find_command: cfn!(&mut ConCommand, &'static CVar, CStr),
    pub find_command_const: cfn!(&ConCommand, &'static CVar, CStr),
}

pub fn get_cvar(name: String) -> OxideResult<&'static mut ConVar> {
    let name = CString::new(name).unwrap();
    let cvar = vmt_call!(interface!(cvar), find_var, name.as_ptr());
    if cvar.is_null() {
        return Err(OxideError::new("invalid cvar"));
    }
    Ok(unsafe { transmute(cvar) })
}
pub fn get_cvar_const(name: String) -> OxideResult<&'static ConVar> {
    let name = CString::new(name).unwrap();
    let cvar = vmt_call!(interface!(cvar), find_var_const, name.as_ptr());
    if cvar.is_null() {
        return Err(OxideError::new("invalid cvar"));
    }
    Ok(unsafe { transmute(cvar) })
}
