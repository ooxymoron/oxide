use std::{
    ffi::{c_char, CStr, CString},
    mem::transmute,
};

use crate::{
    error::{OxideError, OxideResult},
    interface,
    vmt_call,
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
    pub find_command: cfn!(&mut ConCommand, &CVar, CStr),
    _pad1: [usize; 7],

    //pub unsafe fn vfprintf(stream: *mut FILE, format: *const c_char, ap: VaList) -> c_int;
    pub color_console_print: unsafe extern "C" fn(&CVar, &Color, *const c_char, ...),
    pub console_print: unsafe extern "C" fn(&CVar, *const c_char, ...),
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTDisplayFunction {
    pub color_print: cfn!((), &DisplayFunction, &Color, *const c_char),
    pub print: cfn!((), &DisplayFunction, *const c_char),
}

pub type DisplayFunction = WithVmt<VMTDisplayFunction>;

impl CVar {
    pub fn console_print(&self, text: &str) {
        let mut text_raw = [0i8; 8192];
        let c_text = CString::new(text).unwrap();
        text_raw[0..text.len()].copy_from_slice(unsafe { transmute(c_text.to_bytes()) });
        unsafe {
            let function_count = (self as *const _ as *const usize)
                .byte_add(0x38)
                .read_unaligned();

            for i in 0..function_count {
                let display_fn = (self as *const _ as *const *const &DisplayFunction)
                    .byte_add(0x28)
                    .read()
                    .byte_add(i * 8)
                    .read();
                vmt_call!(display_fn, print, text_raw.as_ptr());
            }
        }
    }
}

pub fn get_cvar(name: &str) -> OxideResult<&'static mut ConVar> {
    let name = CString::new(name.to_string()).unwrap();
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
