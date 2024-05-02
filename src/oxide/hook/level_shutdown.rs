use crate::{
    define_hook, get_cheat, oxide::cheat::spread_reduction::SpreadReduction,
    sdk::interfaces::client_mode::ClientMode,
};

fn hook(client_mode: &ClientMode, org: LevelShutdownHook::RawFn) {
    o!().last_entity_cache = None;
    get_cheat!(SpreadReduction).playerperf_send_data = None;
    (org)(client_mode);
}

define_hook!(
    LevelShutdownHook,
    "LevelShutdown",
    hook,
    (),
    (),
    client_mode,
    &ClientMode
);
