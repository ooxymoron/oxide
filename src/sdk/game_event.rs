use std::ffi::c_char;

use std::ffi::CString;

use crate::{cfn, vmt_call};

use super::WithVmt;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTGameEvent {
    _pad: [usize; 2],
    pub get_name: cfn!(*const c_char, &GameEvent),
    pub is_reliable: cfn!(bool, &GameEvent),
    pub is_local: cfn!(bool, &GameEvent),
    pub is_empty: cfn!(bool, &GameEvent, *const c_char),
    pub get_bool: cfn!(bool, &GameEvent, *const c_char, bool),
    pub get_int: cfn!(i32, &GameEvent, *const c_char, i32),
    pub get_float: cfn!(f32, &GameEvent, *const c_char, f32),
    pub get_string: cfn!(*const c_char, &GameEvent, *const c_char, *const c_char),
    pub set_int: cfn!((), &GameEvent, *const c_char, i32),
    pub set_float: cfn!((), &GameEvent, *const c_char, f32),
    pub set_string: cfn!((), &GameEvent, *const c_char, *const c_char),
}

pub type GameEvent = WithVmt<VMTGameEvent>;

impl GameEvent {
    pub fn get_int(&self, name: &str) -> Option<i32>{
        let name = CString::new(name).unwrap();
        let res = vmt_call!(self,get_int,name.as_ptr(),i32::MAX);
        if res == i32::MAX {
            return None;
        }
        return Some(res)
    }
    pub fn get_float(&self, name: &str) -> Option<f32>{
        let name = CString::new(name).unwrap();
        let res = vmt_call!(self,get_float,name.as_ptr(),f32::MAX);
        if res == f32::MAX {
            return None;
        }
        return Some(res)
    }
    pub fn get_bool(&self, name: &str) -> bool{
        let name = CString::new(name).unwrap();
        vmt_call!(self,get_bool,name.as_ptr(),false)
    }
}
