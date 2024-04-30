use crate::cfn;

use super::{bf_read::BfRead, networkable::UnparsedClientClass, view_setup::ViewSetup, WithVmt};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
    _pad1: [usize; 6],
    pub level_init_post_entity: cfn!((), &BaseClient),
    pub level_shutdown: cfn!((), &BaseClient),
    pub get_all_classes: cfn!(&'static UnparsedClientClass, &BaseClient),
    pub(crate) _pad2: [usize; 1],
    pub hud_process_input: cfn!((), &BaseClient, bool),
    pub hud_update: cfn!((), &BaseClient, bool),
    _pad3: [usize; 2],
    pub in_activate_mouse: cfn!((), &BaseClient),
    _pad4: [usize; 20],
    pub frame_stage_notify: cfn!((), &BaseClient, FrameStage),
    pub dispatch_user_message: cfn!(bool, *const u8, u32, &mut BfRead),
    _pad5: [usize; 22],
    pub get_player_view: cfn!(bool, &BaseClient, &ViewSetup),
}
