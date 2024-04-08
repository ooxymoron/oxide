use crate::cfn;

use super::{view_setup::ViewSetup, WithVmt};

#[repr(C)]
#[derive(Debug, Clone,Copy)]
pub enum FrameStage {
    FrameUndefined = -1,
    FrameStart,
    FrameNetUpdateStart,
    FrameNetUpdatePostdataupdateStart,
    FrameNetUpdatePostdataupdateEnd,
    FrameNetUpdateEnd,
    FrameRenderStart,
    FrameRenderEnd,
}

pub type BaseClient = WithVmt<VMTBaseClient>;


#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTBaseClient {
    _pad1: [u32; 6],
    pub level_init_post_entity: cfn!((), &BaseClient),
    pub level_shutdown: cfn!((), &BaseClient),
    _pad2: [u32; 2],
    pub hud_process_input: cfn!((), &BaseClient, bool),
    pub hud_update: cfn!((), &BaseClient, bool),
    _pad3: [u32; 2],
    pub in_activate_mouse: cfn!((), &BaseClient),
    _pad4: [u32; 20],
    pub frame_stage_notify: cfn!((), &BaseClient, FrameStage),
    _pad5: [u32; 23],
    pub get_player_view: cfn!(bool, &BaseClient, &ViewSetup),
}
