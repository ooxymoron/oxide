use std::mem::transmute;

use crate::{
    draw::event::Event,
    log, o,
    sdk::{
        entity::{player::Player, weapon::Weapon},
        game_event::GameEvent,
        user_cmd::{ButtonFlags, UserCmd},
    },
    setting, vmt_call,
};

use super::Cheat;

//const BUCKET_CAP: f32 = 1000.0;
const WEAPON_RANDOM_RANGE: i32 = 10000;

const TF_DAMAGE_CRIT_CHANCE: f32 = 0.02;
//const TF_DAMAGE_CRIT_CHANCE_RAPID: f32 = 0.02;
//const TF_DAMAGE_CRIT_DURATION_RAPID: f32 = 2.0;
//const TF_DAMAGE_CRIT_CHANCE_MELEE: f32 = 0.15;
//
//const TF_DAMAGE_CRITMOD_MAXTIME: i32 = 20;
//const TF_DAMAGE_CRITMOD_MINTIME: i32 = 2;
//const TF_DAMAGE_CRITMOD_DAMAGE: i32 = 800;
//const TF_DAMAGE_CRITMOD_MAXMULT: i32 = 6;
//
const TF_DAMAGE_CRIT_MULTIPLIER: f32 = 3.0;
//const TF_DAMAGE_MINICRIT_MULTIPLIER: f32 = 1.35;

#[derive(Debug)]
pub struct CritManipulation {
    crit_damage: f32,
    melee_damage: f32,
    pub crit_key_pressed: bool,
    last_cmd_spoofed: i32,
}

impl CritManipulation {
    pub fn init() -> CritManipulation {
        CritManipulation {
            crit_damage: 0.0,
            melee_damage: 0.0,
            crit_key_pressed: false,
            last_cmd_spoofed: 0,
        }
    }
    pub fn reset(&mut self) {
        self.crit_damage = 0.0;
        self.melee_damage = 0.0;
        self.last_cmd_spoofed = 0;
    }
    pub fn player_hurt(&mut self, event: &GameEvent) {
        let Ok(p_local) = Player::get_local() else {return};
        let Ok(attacker) = Player::get_byt_user_id(event.get_int("attacker").unwrap()) else {return};
        if attacker.as_ent() != p_local.as_ent() {
            return;
        }
        let attacked = Player::get_byt_user_id(event.get_int("userid").unwrap()).unwrap();
        if attacker.as_ent() == attacked.as_ent() {
            return;
        }
        let crit = event.get_bool("crit");
        let mut damage = event.get_float("damageamount").unwrap();
        let weaponid = event.get_int("weaponid").unwrap();
        let health = event.get_int("health").unwrap();

        if damage > health as f32 {
            damage = health as f32
        }
        let weapon = p_local
            .get_weapon_by_id(unsafe { transmute(weaponid) })
            .unwrap();
        if vmt_call!(weapon, get_slot) == 2 {
            self.melee_damage += damage;
        } else if crit {
            self.crit_damage += damage;
        }
    }
    pub fn find_crit_cmd(
        &mut self,
        curr_cmd_number: i32,
        owner: &mut Player,
        weapon: &mut Weapon,
    ) -> Option<i32> {
        let weapon_index = vmt_call!(weapon.as_ent(), get_index);
        let owner_index = vmt_call!(owner.as_ent(), get_index);
        let index_seed_mask = (weapon_index << 8) | owner_index;

        let crit_chance =
            owner.get_crit_mult() * TF_DAMAGE_CRIT_CHANCE * WEAPON_RANDOM_RANGE as f32;
        dbg!(crit_chance);

        for cmd_number in curr_cmd_number..curr_cmd_number + 100000 {
            let seed = ((o!().util.md5_pseudorandom)(cmd_number) & i32::MAX) ^ index_seed_mask;
            (o!().util.random_seed)(seed);
            let random = (o!().util.random_int)(0, WEAPON_RANDOM_RANGE - 1);
            //if random == 0 {
            //    return Some(cmd_number);
            //}
            if (random as f32) < crit_chance {
                return Some(cmd_number);
            }
        }
        None
    }
    pub fn crits(&mut self, weapon: &mut Weapon) -> i32 {
        let mut weapon_damage = weapon.get_info().weapon_data[weapon.get_mode()].damage;
        if let Ok(gun) = weapon.as_gun() {
            weapon_damage *= gun.get_bullets()
        }
        dbg!(weapon.get_crit_bucket(), TF_DAMAGE_CRIT_MULTIPLIER * weapon_damage as f32);
        dbg!(weapon.get_crit_checks());
        dbg!(weapon.get_crit_seed_requests());
        (*weapon.get_crit_bucket() / (TF_DAMAGE_CRIT_MULTIPLIER * weapon_damage as f32)).floor() as i32
    }
    pub fn damage_till_crit(&mut self, weapon: &mut Weapon) -> Option<f32> {
        let p_local = Player::get_local().unwrap();
        let crit_chance = p_local.get_crit_mult() * TF_DAMAGE_CRIT_CHANCE;

        let observed_chance = self.get_observed_crit_chance(weapon);
        let needed_chance = crit_chance + 0.1;

        if observed_chance <= needed_chance {
            return None;
        }

        //transformed formula for crit check
        let needed_damage =
            (self.crit_damage * (1.2 + needed_chance)) / (TF_DAMAGE_CRIT_MULTIPLIER * (needed_chance + 0.1));

        let resource = p_local.get_resource_data();
        let ranged_damage = resource.damage as f32 - self.melee_damage;
        Some(needed_damage - ranged_damage)
    }
    pub fn get_observed_crit_chance(&self, _: &mut Weapon) -> f32 {
        let p_local = Player::get_local().unwrap();
        let resource = p_local.get_resource_data();

        let ranged_damage = resource.damage as f32 - self.melee_damage;
        if ranged_damage == resource.damage as f32 {
            return 0.0;
        }

        let normalized_crit_damage = self.crit_damage / TF_DAMAGE_CRIT_MULTIPLIER;

        normalized_crit_damage / (normalized_crit_damage + ranged_damage - self.crit_damage)
    }
    pub fn create_move(&mut self, cmd: &mut UserCmd) {
        let p_local = Player::get_local().unwrap();
        let weapon = p_local.weapon();
        let crits = self.crits(weapon);
        let damage_till_crit = self.damage_till_crit(weapon);
        if let Some(damage_till_crit) = damage_till_crit {
            log!("damage till crit: {}", damage_till_crit);
        } else if self.crit_key_pressed
            && cmd.buttons.get(ButtonFlags::InAttack)
            && crits > 0
            && p_local.can_attack()
        {
            log!(
                "damage till crit: {:?}\n crits: {}",
                damage_till_crit,
                crits
            );
            let cmd_num = self
                .find_crit_cmd(
                    cmd.command_number.max(self.last_cmd_spoofed + 1),
                    p_local,
                    weapon,
                )
                .unwrap();
            cmd.command_number = cmd_num;
            cmd.seed = (o!().util.md5_pseudorandom)(cmd_num) & i32::MAX;
            self.last_cmd_spoofed = cmd_num;
        }
    }
}

impl Cheat for CritManipulation {
    fn handle_event(&mut self, event: &mut Event) {
        let crit_key = setting!(aimbot, crit_key);
        if event.is_key_down(&crit_key) {
            self.crit_key_pressed = true
        }
        if event.is_key_up(&crit_key) {
            self.crit_key_pressed = false
        }
    }
}
