use std::ffi::c_char;

use crate::{math::angles::Angles, vmt_call};

use self::net_channel::NetChannel;

use super::*;

pub type BaseEngine = WithVmt<VMTBaseEngine>;

const MAX_PLAYER_NAME_LENGTH: usize = 32;
const SIGNED_GUID_LEN: usize = 32;
const MAX_CUSTOM_FILES: usize = 4;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlayerInfoUnparsed {
    pub name: [u8; MAX_PLAYER_NAME_LENGTH],
    pub user_id: i32,
    pub guid: [u8; SIGNED_GUID_LEN + 1],
    pub friends_id: u32,
    pub friends_name: [u8; MAX_PLAYER_NAME_LENGTH],
    pub fakeplayer: bool,
    pub ishltv: bool,
    pub custom_files: [u32; MAX_CUSTOM_FILES],
    pub files_downloaded: c_char,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub user_id: i32,
    pub guid: String,
    pub friends_id: u32,
    pub friends_name: String,
    pub fakeplayer: bool,
    pub ishltv: bool,
    pub custom_files: [u32; MAX_CUSTOM_FILES],
    pub files_downloaded: c_char,
}

impl From<PlayerInfoUnparsed> for PlayerInfo {
    fn from(value: PlayerInfoUnparsed) -> Self {
        let str_from_arr = |arr: Vec<u8>| -> String {
            unsafe {
                String::from_utf8_unchecked(arr)
                    .chars()
                    .filter(|char| *char != '\0')
                    .collect()
            }
        };
        PlayerInfo {
            name: str_from_arr(value.name.to_vec()),
            user_id: value.user_id,
            guid: str_from_arr(value.guid.to_vec()),
            friends_id: value.friends_id,
            friends_name: str_from_arr(value.friends_name.to_vec()),
            fakeplayer: value.fakeplayer,
            ishltv: value.ishltv,
            custom_files: value.custom_files,
            files_downloaded: value.files_downloaded,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTBaseEngine {
    _pad1: [usize; 5],
    pub get_screen_size: cfn!((), &BaseEngine, &isize, &isize),
    _pad2: [usize; 2],
    pub get_player_info: cfn!(bool, &BaseEngine, isize, &mut PlayerInfoUnparsed),
    _pad3: [usize; 2],
    pub get_player_from_user_id: cfn!(u32, &BaseEngine, u32),
    pub get_local_player: cfn!(u32, &BaseEngine),
    _pad4: [usize; 6],
    pub get_view_angles: cfn!((), &BaseEngine, Angles),
    pub set_view_angles: cfn!((), &BaseEngine, Angles),
    pub get_max_clients: cfn!(isize, &BaseEngine),
    _pad5: [usize; 4],
    pub is_in_game: cfn!(bool, &BaseEngine),
    pub is_connected: cfn!(bool, &BaseEngine),
    _pad6: [usize; 8],
    pub world_to_screen_matrix: cfn!(VMatrix, &BaseEngine),
    _pad7: [usize; 35],
    pub get_net_channel_info: cfn!(*const NetChannel, &BaseEngine),
    _pad8: [usize; 33],
    pub send_cmd_unrestricted: cfn!((), &BaseEngine, *const c_char),
    _pad9: [usize; 21],
    pub server_cmd_key_values: cfn!((),()),
}
impl BaseEngine {
    pub fn get_net_channel(&self) -> Option<&NetChannel> {
        let net_channel = vmt_call!(self, get_net_channel_info);
        if net_channel.is_null() {
            return None;
        }
        Some(unsafe { transmute(net_channel) })
    }
}
