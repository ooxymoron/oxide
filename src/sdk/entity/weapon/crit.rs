use std::mem::transmute;

use crate::{math::remap_clamped, o, oxide::cheat::crit_manipulation::{BUCKET_CAP, CRIT_DURATION_RAPID, CRIT_MULTIPLIER}};

use super::{HasNetvars, Weapon};


impl Weapon {
    pub fn get_next_check(&self) -> Option<f32> {
        let owner = self.get_owner().resolve().unwrap().as_player().unwrap();
        let now = o!().global_vars.interval_per_tick * *owner.get_tick_base() as f32;
        if self.get_info().weapon_data[self.get_mode()].use_rapid_fire_crits {
            if now < *self.get_last_crit_check_time() + 1.0 {
                return Some(*self.get_last_crit_check_time() + 1.0 - now);
            } else if now < *self.get_last_rapid_fire_crit_check_time() + 1.0
            {
                return Some(
                    *self.get_last_rapid_fire_crit_check_time() + 1.0 - now,
                );
            };
            return None;
        }
        None
    }
    pub fn get_crit_time(&self) -> Option<f32> {
        if *self._get_crit_time() > o!().global_vars.curtime {
            Some((*self._get_crit_time() - o!().global_vars.curtime) / CRIT_DURATION_RAPID)
        } else {
            None
        }
    }
    pub fn crits(&mut self) -> (i32, i32) {
        let data = self.get_info().weapon_data[self.get_mode()].clone();
        let mut self_damage = data.damage;
        if let Ok(gun) = self.as_gun() {
            self_damage *= gun.get_bullets()
        }
        let mut damage = self_damage as f32;
        if data.use_rapid_fire_crits {
            damage *= CRIT_DURATION_RAPID / data.time_fire_delay;
        }
        if damage * CRIT_MULTIPLIER > BUCKET_CAP {
            damage = BUCKET_CAP as f32 / CRIT_MULTIPLIER
        }
        damage *= if self.get_info().melee_weapon {
            0.5
        } else {
            remap_clamped((*self.get_crit_seed_requests() as f32)/ *self.get_crit_checks() as f32, 0.1, 1., 1., 3.)
        };
        damage *= CRIT_MULTIPLIER;
        (
            ((*self.get_crit_bucket() + self_damage as f32)/ damage) as i32,
            (BUCKET_CAP / damage) as i32,
        )
    }
    pub fn get_last_rapid_fire_crit_check_time(&self) -> &mut f32 {
        let netvar = self
            .get_netvar(["LocalActiveTFWeaponData", "m_flLastCritCheckTime"])
            .unwrap();
        unsafe {
            transmute::<_, &mut f32>(
                (self as *const _ as *const f32).byte_add(netvar.offset + 4 * 3),
            )
        }
    }
    fn _get_crit_time(&self) -> &mut f32 {
        let netvar = self
            .get_netvar(["LocalActiveTFWeaponData", "m_flLastCritCheckTime"])
            .unwrap();
        unsafe {
            transmute::<_, &mut f32>((self as *const _ as *const f32).byte_add(netvar.offset - 4))
        }
    }
}
