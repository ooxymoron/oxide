use crate::{
    call_original, cfn, get_cheat, interface, log, o,
    oxide::cheat::crit_manipulation::CritManipulation, sdk::entity::weapon::Weapon,
};

pub const NAME: &str = "CalcIsAttackCritical";

pub type CalcIsAttackCritical = cfn!(bool, &mut Weapon);

pub extern "C" fn hook(weapon: &mut Weapon) -> bool {
    if let Some(seed) = get_cheat!(CritManipulation).last_spoofed_cmd_seed && (interface!(prediction).first_time_predicted || !interface!(prediction).in_prediction){
        *o!().prediction_seed = seed;
        weapon.current_seed = 0;
        get_cheat!(CritManipulation).last_spoofed_cmd_seed = None;
        let res = call_original!(NAME, CalcIsAttackCritical, weapon);
        log!("spoofing calc {}, {}", seed, res);
        return res 
    }
    return false;
}
