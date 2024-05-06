use self::{entity::player::Player, user_cmd::UserCmd, view_setup::ViewSetup};

use super::*;

pub type ClientMode = WithVmt<VMTClientMode>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTClientMode {
    _pad1: [usize; 13],
    pub should_draw_entity: cfn!(bool, &mut ClientMode, &mut Entity),
    pub should_draw_local_player: cfn!(bool, &mut ClientMode, &mut Player),
    _pad2: [usize; 2],
    pub override_view: cfn!((), &mut ClientMode, &mut ViewSetup),
    _pad3: [usize; 4],
    pub create_move: cfn!(bool, &mut ClientMode, f32, &mut UserCmd),
    pub level_init: cfn!((), &mut ClientMode, *const u8),
    pub level_shutdown: cfn!((), &mut ClientMode),
    pub should_draw_view_model: cfn!(bool, &mut ClientMode),
}
