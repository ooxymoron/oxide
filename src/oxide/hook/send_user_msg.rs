use crate::{
    define_hook, get_cheat,
    oxide::cheat::spread_reduction::{self, SpreadReduction},
    sdk::{
        net_channel::{self, NetChannel, NetMessage},
        HasVmt,
    },
    util::{
        debug::print_module_addres_offset,
        handles::{CLIENT, ENGINE},
    },
};

unsafe fn hook(
    net_channel: &NetChannel,
    message: &NetMessage,
    mut force_reliable: bool,
    voice: bool,
    org: SendUserMessageHook::RawFn,
) -> bool{
    force_reliable = get_cheat!(SpreadReduction).send_user_msg_pre(message);
    let res = (org)(net_channel, message, force_reliable, voice);
    //get_cheat!(SpreadReduction).send_user_msg_post(message);
    res
}

define_hook!(
    SendUserMessageHook,
    "SendUserMessage",
    hook,
    bool,
    true,
    channel,
    &NetChannel,
    messaage,
    &NetMessage,
    force_reliable,
    bool,
    voice,
    bool
);
