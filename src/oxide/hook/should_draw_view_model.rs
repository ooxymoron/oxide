
use crate::{
    define_hook,
    sdk::{condition::ConditionFlags, entity::player::Player, interfaces::client_mode::ClientMode}, setting,
};

fn hook(client_move: &mut ClientMode, org: ShouldDrawViewModelHook ::RawFn) -> bool {
    if let Ok(plocal) = Player::get_local() {
        if *setting!(visual,remove_zoom) && plocal.get_condition().get(ConditionFlags::Zoomed) {
            return true
        }
    }
    (org)(client_move)
}

define_hook!(
    ShouldDrawViewModelHook,
    "ShouldDrawViewModel",
    hook,
    bool,
    true,
    client_mode,
    &mut ClientMode
);
