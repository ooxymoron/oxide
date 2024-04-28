use crate::{cfn, impl_has_vmt};
use std::{fmt::Debug, mem::transmute};

pub use derivative::Derivative;

pub mod camerd_third_data;
pub mod collideable;
pub mod condition;
pub mod convar;
pub mod entity;
pub mod font;
pub mod input;
pub mod networkable;
pub mod user_cmd;
pub mod view_setup;
pub mod attribute_manager;
pub mod interfaces;
pub mod global_vars;

pub type CBaseHandle = u32;
pub type ConCommand = *const u8;

pub type VMatrix = [[f32; 4]; 4];

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WithVmt<T: 'static> {
    pub vmt: *mut T,
}

pub trait HasVmt<T: 'static> {
    fn get_vmt(&self) -> &'static T;
    fn set_vmt(&mut self, vmt: *mut T);
}

impl<T: 'static + Clone + Debug> HasVmt<T> for WithVmt<T> {
    fn get_vmt(&self) -> &'static T {
        unsafe { transmute(self.vmt) }
    }
    fn set_vmt(&mut self, vmt: *mut T) {
        self.vmt = vmt
    }
}
