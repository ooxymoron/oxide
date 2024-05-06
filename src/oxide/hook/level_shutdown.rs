use crate::{
    define_hook, get_cheat,
    oxide::cheat::spread_reduction::{seed_prediction::State, SpreadReduction},
    sdk::interfaces::base_client::BaseClient,
};

fn hook(base_client: &BaseClient, org: LevelShutdownHook::RawFn) {
    o!().last_entity_cache = None;
    get_cheat!(SpreadReduction).playerperf_send_data = None;
    get_cheat!(SpreadReduction).state = State::UNSYNCED;
    (org)(base_client);
}

define_hook!(
    LevelShutdownHook,
    "LevelShutdown",
    hook,
    (),
    (),
    base_client,
    &BaseClient
);
