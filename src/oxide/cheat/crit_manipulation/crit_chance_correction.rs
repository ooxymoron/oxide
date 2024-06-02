use crate::{oxide::cheat::crit_manipulation::CRIT_MULTIPLIER, sdk::entity::player::Player};

use super::CritManipulation;

impl CritManipulation {
    pub fn calc_observed_crit_chance(&self) -> f32 {
        let p_local = Player::get_local().unwrap();
        let resource = p_local.get_resource_data();

        let ranged_damage = resource.damage as f32 - self.melee_damage;
        if ranged_damage == 0.0 {
            return 0.0;
        }

        let normalized_crit_damage = self.crit_damage / CRIT_MULTIPLIER;

        normalized_crit_damage / (normalized_crit_damage + ranged_damage - self.crit_damage)
    }
}
