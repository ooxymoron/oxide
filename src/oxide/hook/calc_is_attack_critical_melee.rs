use crate::{
    call_original, cfn, get_cheat, oxide::cheat::crit_manipulation::CritManipulation,
    sdk::entity::weapon::Weapon,
};

pub const NAME: &str = "CalcIsAttackCriticalMelee";

pub type CalcIsAttackCriticalMelee = cfn!(bool, &mut Weapon);

pub extern "C" fn hook(weapon: &mut Weapon) -> bool {
    if !get_cheat!(CritManipulation).respoof_seed(weapon) {
        return false;
    }
    call_original!(NAME, CalcIsAttackCriticalMelee, weapon)
}
