use crate::call_original;

pub const NAME: &str = "ClSendMove";

pub type ClSendMove = extern "C" fn();

pub extern "C" fn hook() {
    call_original!(NAME, ClSendMove);
}
