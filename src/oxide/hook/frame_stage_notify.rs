use crate::{
    define_hook,
    oxide::{cheat::visual::Visuals, entity_cache::EntityCache},
    sdk::interfaces::base_client::{BaseClient, FrameStage},
};

fn hook(client: &BaseClient, stage: FrameStage, org: FrameStageNotifyHook::RawFn) {

    match stage {
        FrameStage::FrameNetUpdateEnd => {
            match EntityCache::init() {
                Ok(cache) => {
                    o!().last_entity_cache = Some(cache);
                }
                Err(_) => {
                    o!().last_entity_cache = None;
                }
            };
            let mut visuals = o!().cheats.get::<Visuals>(Visuals::name());
            visuals.net_update_end().unwrap();
        }
        _ => {}
    }
    (org)(client,stage);
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
