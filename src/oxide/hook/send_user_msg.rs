use crate::{
    define_hook, get_cheat,
    oxide::cheat::spread_reduction::SpreadReduction,
    sdk::net_channel::{NetChannel, NetMessage},
};

unsafe fn hook(
    net_channel: &NetChannel,
    message: &NetMessage,
    force_reliable: bool,
    voice: bool,
    org: SendUserMessageHook::RawFn,
) -> bool {
    let syncing_nospread = get_cheat!(SpreadReduction).should_sync_delta(message);
    let res = (org)(net_channel, message, force_reliable | syncing_nospread, voice);
    if syncing_nospread {
        get_cheat!(SpreadReduction).send_user_msg_post();
    }
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
