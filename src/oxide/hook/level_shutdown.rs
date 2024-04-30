use crate::{define_hook, sdk::interfaces::client_mode::{self, ClientMode}};

fn hook(client_mode: &ClientMode, org: LevelShutdownHook::RawFn) {
    o!().last_entity_cache = None;
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
