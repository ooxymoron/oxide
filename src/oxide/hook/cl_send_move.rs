use crate::{get_cheat, oxide::cheat::spread_reduction::SpreadReduction};

pub const NAME: &str = "ClSendMove";

pub type ClSendMove = extern "C" fn();

pub extern "C" fn hook() {
    get_cheat!(SpreadReduction).cl_send_move();
}
