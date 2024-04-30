use crate::{
    call_original, cfn, get_cheat,  oxide::cheat::spread_reduction::SpreadReduction,
    sdk::bf_read::BfRead,
};

pub const NAME: &str = "DispatchUserMessage";

pub type DispatchUserMessage = cfn!(bool, *const u8, u32, &mut BfRead);

pub extern "C" fn hook(this: *const u8, msg_type: u32, buffer: &mut BfRead) -> bool {
    //buffer.reset();
    //let success = get_cheat!(SpreadReduction).dispatch_user_message(msg_type, buffer);
    //if success {
    //    true
    //} else {
    //    buffer.reset();
    //}
    call_original!(NAME, DispatchUserMessage, this, msg_type, buffer)
}

