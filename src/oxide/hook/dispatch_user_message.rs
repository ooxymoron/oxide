use crate::define_hook;
use crate::sdk::interfaces::base_client::BaseClient;
use crate::{get_cheat, oxide::cheat::spread_reduction::SpreadReduction, sdk::bf_read::BfRead};

pub extern "C" fn hook(
    client: &mut BaseClient,
    msg_type: u32,
    buffer: &mut BfRead,
    org: DispatchUserMessageHook::RawFn,
) -> bool {
    buffer.reset();
    if get_cheat!(SpreadReduction).dispatch_user_message(msg_type, buffer) {
        return false; 
    };

    (org)(client, msg_type, buffer)
}

define_hook!(
    DispatchUserMessageHook,
    "DispatchUserMessage",
    hook,
    bool,
    false,
    client,
    &mut BaseClient,
    msg_typ,
    u32,
    buffer,
    &mut BfRead
);
