use crate::{
    define_hook,
    sdk::{condition::ConditionFlags, entity::player::Player, interfaces::client_mode::ClientMode}, setting,
};

fn hook(client_move: &mut ClientMode, org: ShouldDrawLocalPlayerHook::RawFn) -> bool {
    if let Ok(plocal) = Player::get_local() {
        if setting!(visual,third_person) && plocal.get_condition().get(ConditionFlags::Zoomed) {
            return true
        }
    }
    (org)(client_move)
}

define_hook!(
    ShouldDrawLocalPlayerHook,
    "ShouldDrawLocalPlayer",
    hook,
    bool,
    true,
    client_mode,
    &mut ClientMode
);