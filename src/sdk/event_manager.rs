use crate::cfn;

use super::{game_event::GameEvent, WithVmt};


#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTGameEventManager {
    _pad: [usize; 9],
    pub fire_event: cfn!(bool, &GameEventManager, &GameEvent),
}

pub type GameEventManager = WithVmt<VMTGameEventManager>;
