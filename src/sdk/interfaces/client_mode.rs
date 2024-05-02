use self::{user_cmd::UserCmd, view_setup::ViewSetup};

use super::*;

pub type ClientMode = WithVmt<VMTClientMode>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTClientMode {
    _pad1: [usize; 10],
    pub should_draw_local_player: cfn!(bool, &mut ClientMode),
    pub should_draw_view_model: cfn!(bool, &mut ClientMode),
    _pad2: [usize; 5],
    pub pre_render: cfn!((), &mut ClientMode, &mut ViewSetup),
    _pad3: [usize; 4],
    pub create_move: cfn!(bool, &mut ClientMode, f32, &mut UserCmd),
    pub level_init: cfn!((), &mut ClientMode, *const u8),
    pub level_shutdown: cfn!((), &mut ClientMode),
}
