use std::mem::transmute;

use libc::c_char;
use std::ffi::CString;

use crate::util::{
    handles::ENGINE,
    sigscanner::find_sig,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ClientState {}
static mut SEND_STRING_CMD_FUNCIOTN: Option<extern "C" fn(&ClientState, *const c_char)> = None;
impl ClientState {
    pub fn send_string_cmd(&self, text: &str) {
        unsafe {
            if SEND_STRING_CMD_FUNCIOTN.is_none() {
                SEND_STRING_CMD_FUNCIOTN = Some(transmute(
                    find_sig::<u8>(ENGINE, "48 8B 7F 20 48 85 FF ? ? ? ? ? ? 55 66 0F EF C0 31 C9 31 D2 48 89 E5 41 55 41").unwrap(),
                ));
            }
            let function = SEND_STRING_CMD_FUNCIOTN.unwrap();
            let name = CString::new(text).unwrap();
            function(&self, name.as_ptr());
        }
    }
}
//
