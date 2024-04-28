use crate::{
    define_hook,
    oxide::{cheat::visual::Visuals, entity_cache::EntityCache},
    sdk::interfaces::base_client::{BaseClient, FrameStage},
};

fn subhooks(hook: &mut FrameStageNotifyHook) {
    hook.before = Some(|_, stage| {
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
        None
    });
}

define_hook!(
    FrameStageNotifyHook,
    "FrameStageNotify",
    (),
    (),
    subhooks,
    client,
    &BaseClient,
    stage,
    FrameStage
);
