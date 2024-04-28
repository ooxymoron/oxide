use std::ffi::c_char;

use crate::cfn;

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
