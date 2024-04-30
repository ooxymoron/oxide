use crate::{cfn, impl_has_vmt};
use std::{fmt::Debug, mem::transmute};

pub use derivative::Derivative;

use self::entity::Entity;

pub mod attribute_manager;
pub mod camerd_third_data;
pub mod collideable;
pub mod condition;
pub mod convar;
pub mod effect_data;
pub mod entity;
pub mod fire_bullets_info;
pub mod font;
pub mod game_event;
pub mod global_vars;
pub mod input;
pub mod interfaces;
pub mod networkable;
pub mod user_cmd;
pub mod view_setup;
pub mod bf_read;
pub mod user_message;
pub mod net_channel;
pub mod client_state;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct EntHandle(u32);

const INVALID_HANDLE: u32 = 0xffffffff;

impl EntHandle {
    pub fn resolve(self) -> Option<&'static mut Entity> {
        if self.0 == INVALID_HANDLE {
            return None;
        }
        Entity::get_ent_from_handle(self)
    }
}
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
