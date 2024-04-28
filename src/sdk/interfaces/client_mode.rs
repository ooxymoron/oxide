use self::{user_cmd::UserCmd, view_setup::ViewSetup};

use super::*;

pub type ClientMode = WithVmt<VMTClientMode>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTClientMode {
    _pad1: [cfn!((),()); 17],
    pub override_view: cfn!((), &mut ClientMode, &mut ViewSetup),
    _pad2: [cfn!((),()); 4],
    pub create_move: cfn!(bool, &mut ClientMode, f32, &mut UserCmd),
    pub level_init: cfn!((), &mut ClientMode, *const u8),
    pub level_shutdown: cfn!((), &mut ClientMode),
}
