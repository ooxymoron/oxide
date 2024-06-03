use crate::{
    define_hook, get_cheat,
    oxide::{cheat::visual::Visuals, entity_cache::EntityCache},
    sdk::interfaces::base_client::{BaseClient, FrameStage},
};

fn hook(client: &BaseClient, stage: FrameStage, org: FrameStageNotifyHook::RawFn) {
    match stage {
        FrameStage::FrameNetUpdateEnd => {
            match EntityCache::init() {
                Ok(cache) => {
                    o!().player_resource_manager.update(&cache);
                    o!().last_entity_cache = Some(cache);
                }
                Err(_) => {
                    o!().last_entity_cache = None;
                }
            };
            let mut visuals = get_cheat!(Visuals);
            visuals.net_update_end().unwrap();
        }
        _ => {}
    }
    (org)(client, stage);
}

define_hook!(
    FrameStageNotifyHook,
    "FrameStageNotify",
    hook,
    (),
    (),
    client,
    &BaseClient,
    stage,
    FrameStage
);
