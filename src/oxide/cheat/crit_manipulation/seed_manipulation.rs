use crate::{o, sdk::entity::weapon::Weapon, vmt_call};

use super::{CritManipulation, CMD_SCAN_LIMIT, WEAPON_RANDOM_RANGE};

impl CritManipulation {
    pub fn my_find_cmd(
        &mut self,
        curr_cmd_number: i32,
        weapon: &mut Weapon,
        crit: bool,
    ) -> Option<(i32, i32)> {
        let owner = weapon.get_owner().resolve().unwrap().as_player().unwrap();
        let weapon_index = vmt_call!(weapon.as_ent(), get_index);
        let owner_index = vmt_call!(owner.as_ent(), get_index);
        let mut index_seed_mask = (weapon_index << 8) | owner_index;
        if weapon.get_info().melee_weapon {
            index_seed_mask = index_seed_mask << 8
        }

        let crit_chance = self.calc_crit_chance(weapon) * WEAPON_RANDOM_RANGE as f32;

        for cmd_number in curr_cmd_number..curr_cmd_number + CMD_SCAN_LIMIT {
            let seed = (o!().util.md5_pseudorandom)(cmd_number) & i32::MAX;
            (o!().util.random_seed)(seed ^ index_seed_mask);
            let random = (o!().util.random_int)(0, WEAPON_RANDOM_RANGE - 1);
            if ((random as f32) < crit_chance) == crit {
                return Some((cmd_number, seed));
            }
        }
        None
    }
}
