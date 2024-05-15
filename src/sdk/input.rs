use crate::{cfn, impl_has_vmt};

use super::user_cmd::UserCmd;

pub const MULTIPLAYER_BACKUP: usize = 90;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTInput {
    _pad: [usize; 18],
    pub activate_mouse: cfn!((), &Input),
    pub deactivate_mouse: cfn!((), &Input),
}
pub struct Input {
    vmt: *const VMTInput,
    _pad: [i8; 0x100],
    pub commands: *const UserCmd,
}

impl_has_vmt!(Input, VMTInput);
