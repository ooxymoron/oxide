use std::{
    ffi::{c_char, CStr, CString, VaList},
    mem::transmute,
};

use crate::{
    error::{OxideError, OxideResult},
    interface, vmt_call,
};

use self::convar::ConVar;

use super::*;

pub type CVar = WithVmt<VMTCVar>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub a: i32,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTCVar {
    _pad: [usize; 12],
    find_var: cfn!(*mut ConVar, &CVar, *const i8),
    pub find_var_const: cfn!(*const ConVar, &CVar, *const i8),
    pub find_command: cfn!(&mut ConCommand, &'static CVar, CStr),
    _pad1: [usize; 7],

    //pub unsafe fn vfprintf(stream: *mut FILE, format: *const c_char, ap: VaList) -> c_int;
    pub color_console_print: unsafe extern "C-unwind" fn(&'static CVar, &Color, *const c_char, ...)
}
pub type DispalyFunctionType = cfn!((), *const *const u8, &Color, *const i8);
impl CVar {
    pub fn console_print(&self, color: &Color, text: *const i8) {
        unsafe {
            dbg!((self as *const _ as *const usize).byte_add(0x38).read_unaligned());
            let function_count = (self as *const _ as *const usize).byte_add(0x38).read_unaligned();

            for i in 0..function_count {
                let display_fn = *(self as *const _ as *const *const *const DispalyFunctionType)
                    .byte_add(0x28 + i * 8);
                dbg!(display_fn.read_unaligned());
                (**display_fn)(transmute(display_fn), color, text)
            }
        }
    }
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
