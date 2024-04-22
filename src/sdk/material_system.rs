use std::ffi::CStr;

use libc::c_void;

use super::*;

pub type MaterialSystem = WithVmt<VMTMaterialSystem>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTMaterialSystem {
    _pad1: [u8;4 * 73],
    pub find_material: cfn!(&'static IMaterial, &'static MaterialSystem , CStr, CStr, bool, &CStr),
    _pad2: [u8;4 * 26],
    pub get_render_context: cfn!(&'static IMatRenderContext,&'static MaterialSystem),
}

pub type IMaterial = WithVmt<VMTIMaterial>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTIMaterial {
    _pad1: [u8;4 * 27],
    pub alpha_modulate: cfn!(c_void, &'static IMaterial, f32),
    pub color_modulate: cfn!(c_void, &'static IMaterial, f32, f32, f32),
    pub set_material_var_flag: cfn!(c_void, &'static IMaterial, isize,bool), 
    pub get_material_var_flag: cfn!(bool, &'static IMaterial, isize), 
}

pub type IMatRenderContext = WithVmt<VMTIMatRenderContext>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTIMatRenderContext {
    _pad1: [u8;4 * 11],
    pub depth_range: cfn!(c_void, &'static IMatRenderContext, f32,f32),
}
