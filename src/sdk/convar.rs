use std::ffi::CStr;

use derivative::Derivative;
use libc::c_void;

use crate::cfn;


#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTConVar {
    _pad: [i32; 14],
    pub internal_set_value: cfn!((), &'static ConVar , &CStr),
    pub internal_set_float_value: cfn!((), &'static ConVar,f32 , bool),
    pub internal_set_int_value: cfn!((), &'static ConVar, isize),
}

#[repr(C)]
#[derive(Derivative,Clone)]
#[derivative(Debug)]
pub struct ConVar {
    #[derivative(Debug="ignore")]
    pub vmt: &'static VMTConVar,
    #[derivative(Debug="ignore")]
    _pad: [u8; 0x18],
    #[derivative(Debug="ignore")]
    pub parent: &'static ConVar,
    pub default_value: *const i8,
    pub string: *const i8,
    pub string_length: isize,
    pub float_value: f32,
    pub int_value: isize,
    pub has_min: bool,
    pub min_val: f32,
    pub has_max: bool,
    pub max_val: f32,
    pub has_comp_min: bool,
    pub comp_min_val: f32,
    pub has_comp_max: bool,
    pub comp_max_val: f32,
    pub competitive_restrictions: bool,
    pub change_callback: &'static c_void,
}
