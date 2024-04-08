use std::{intrinsics::transmute_unchecked, mem::MaybeUninit};

use derivative::Derivative;

use crate::{
    vmt_call,
    error::{OxideError, OxideResult},
    interface,
    math::angles::Angles,
    o,
    sdk::CBaseHandle,
};

use super::{
    base_engine::PlayerInfo, condition::Condition, player_class::PlayerClass, user_cmd::UserCmd,
    Entity,
};

pub const MAX_WEAPONS: usize = 48;

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct VMTPlayer {}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Player {
    pub vmt: *const VMTPlayer,
    #[derivative(Debug = "ignore")]
    _pad6: [u8; 0xC50],
    pub next_attack: f32,
    #[derivative(Debug = "ignore")]
    _pad7: [u8; 0x84],
    pub my_weapons: [CBaseHandle; MAX_WEAPONS],
    #[derivative(Debug = "ignore")]
    _pad8: [u8; 0xD0],
    pub vec_punch_angle: Angles,
    #[derivative(Debug = "ignore")]
    _pad9: [u8; 0xD0],
    pub object_mode: isize,
    #[derivative(Debug = "ignore")]
    _pad10: [u8; 0x1C4],
    pub angle: Angles,
    #[derivative(Debug = "ignore")]
    _pad11: [u8; 0x48],
    pub current_command: *const UserCmd,
    #[derivative(Debug = "ignore")]
    _pad12: [u8; 0xCC],
    pub tick_base: isize, //0x1234
    #[derivative(Debug = "ignore")]
    _pad131: [u8; 0x3f8],
    pub player_class: PlayerClass, //0x162C
    #[derivative(Debug = "ignore")]
    _pad14: [u8; 0x36C],
    pub condition: Condition,
    #[derivative(Debug = "ignore")]
    _pad15: [u8; 0x18],
    pub condition_bits: isize,
    #[derivative(Debug = "ignore")]
    _pad16: [u8; 0x418],
    pub allow_move_during_taunt: bool,
    #[derivative(Debug = "ignore")]
    _pad17: [u8; 0x18],
    pub force_taunt_cam: isize,
}

impl Player {
    pub fn as_ent(&self) -> &mut Entity {
        unsafe { transmute_unchecked(self) }
    }
    pub fn can_attack(&self) -> bool {
        let now = o!().global_vars.now();
        self.next_attack <= now
    }
    pub fn info(&self) -> OxideResult<PlayerInfo> {
        let mut info = unsafe { MaybeUninit::zeroed().assume_init() };
        let id = vmt_call!(self.as_ent().as_networkable(), get_index);
        let res = vmt_call!(interface!(base_engine), get_player_info, id, &mut info);
        if !res {
            return Err(OxideError::new("failed to get player info"));
        }
        Ok(info.into())
    }
}
