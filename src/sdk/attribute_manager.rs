use std::{
    ffi::{c_char, CStr, CString},
    ptr::null,
};

use derivative::Derivative;

use crate::{vmt_call, cfn};

use super::entity::Entity;

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct VMTAttributeManager {
    #[derivative(Debug = "ignore")]
    _pad1: [usize; 0x11],
    pub get_float: cfn!(
        f32,
        *const AttributeManager,
        f32,
        *const Entity,
        *const c_char,
        *const u8
    ),
    pub get_string: cfn!(
        *const c_char,
        *const AttributeManager,
        *const c_char,
        *const Entity,
        *const c_char,
        *const u8
    ),
}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct AttributeManager {
    pub vmt: *const VMTAttributeManager,
}

impl AttributeManager {
    pub fn get_float(&self, name: &str, ent: &Entity, default: f32) -> f32 {
        let name = CString::new(name).unwrap();
        vmt_call!(self, get_float, default, ent, name.as_ptr(), null())
    }
    pub fn get_string(&self, name: &str, ent: &Entity, default: &str) -> String {
        let name = CString::new(name).unwrap();
        let default = CString::new(default).unwrap();

        let res = vmt_call!(
            self,
            get_string,
            default.as_ptr(),
            ent,
            name.as_ptr(),
            null()
        );
        unsafe {
            let res = CStr::from_ptr(res);
            res.to_str().unwrap().to_owned()
        }
    }
}


#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct VMTHasAttributes {
    pub get_attribute_manager: cfn!(&'static AttributeManager,*const HasAttributes),
}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct HasAttributes {
    #[derivative(Debug = "ignore")]
    pub vmt: *const VMTHasAttributes,
}
