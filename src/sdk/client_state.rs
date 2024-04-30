use std::mem::transmute;

use libc::c_char;
use std::ffi::CString;

use crate::{
    cfn,
    util::{handles::ENGINE, sigscanner::find_sig},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ClientState {}
static mut SEND_STRING_CMD_FUNCIOTN: Option<cfn!((), &ClientState, *const c_char)> = None;
impl ClientState {
    pub fn send_string_cmd(&self, text: &str) {
        unsafe {
            if SEND_STRING_CMD_FUNCIOTN.is_none() {
                SEND_STRING_CMD_FUNCIOTN = Some(transmute(
                    find_sig::<u8>(ENGINE, "48 8B 7F ? 48 85 FF 0F 84 ? ? ? ? 55").unwrap(),
                ))
            }
            let function = SEND_STRING_CMD_FUNCIOTN.unwrap();
            let name = CString::new(text).unwrap();
            function(self, name.as_ptr())
        }
    }
}
//
