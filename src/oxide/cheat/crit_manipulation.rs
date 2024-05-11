use std::{borrow::Borrow, collections::HashMap, mem::transmute};

use crate::{
    interface, log, o,
    sdk::{
        entity::{
            player::Player,
            weapon::{
                ids::{ItemDefinitionInex, WeaponId},
                Weapon,
            },
        },
        game_event::GameEvent,
    },
    vmt_call,
};

use super::Cheat;

const BUCKET_CAP: f32 = 1000.0;
const WEAPON_RANDOM_RANGE: i32 = 10000;

const TF_DAMAGE_CRIT_CHANCE: f32 = 0.02;
const TF_DAMAGE_CRIT_CHANCE_RAPID: f32 = 0.02;
const TF_DAMAGE_CRIT_DURATION_RAPID: f32 = 2.0;
const TF_DAMAGE_CRIT_CHANCE_MELEE: f32 = 0.15;

const TF_DAMAGE_CRITMOD_MAXTIME: i32 = 20;
const TF_DAMAGE_CRITMOD_MINTIME: i32 = 2;
const TF_DAMAGE_CRITMOD_DAMAGE: i32 = 800;
const TF_DAMAGE_CRITMOD_MAXMULT: i32 = 6;

const TF_DAMAGE_CRIT_MULTIPLIER: f32 = 3.0;
const TF_DAMAGE_MINICRIT_MULTIPLIER: f32 = 1.35;

#[derive(Debug)]
pub struct CritManipulation {
    buckets: HashMap<i32, f32>,
    crit_cmd_numbers: HashMap<ItemDefinitionInex, Vec<i32>>,
    crit_damage: f32,
    melee_damage: f32,
}

impl CritManipulation {
    pub fn init() -> CritManipulation {
        CritManipulation {
            buckets: HashMap::new(),
            crit_cmd_numbers: HashMap::new(),
            crit_damage: 0.0,
            melee_damage: 0.0,
        }
    }
    pub fn player_hurt(&mut self, event: &GameEvent) {
        let Ok(p_local) = Player::get_local() else {return};
        let attacker = Player::get_byt_user_id(event.get_int("attacker").unwrap()).unwrap();
        if attacker.as_ent() != p_local.as_ent() {
            return;
        }
        let attacked = Player::get_byt_user_id(event.get_int("userid").unwrap()).unwrap();
        if attacker.as_ent() == attacked.as_ent() {
            return
        }
        let crit = event.get_bool("crit");
        let mut damage = event.get_float("damageamount").unwrap();
        let weaponid = event.get_int("weaponid").unwrap();
        let health = event.get_int("health").unwrap();

        if damage > health as f32 {
            damage = health as f32
        }
        dbg!(weaponid);
        let weapon = p_local
            .get_weapon_by_id(unsafe { transmute(weaponid) })
            .unwrap();
        dbg!(vmt_call!(weapon, get_slot),weapon.get_item_definition_index());
        if vmt_call!(weapon, get_slot) == 2 {
            self.melee_damage += damage;
        } else if crit {
            self.crit_damage += damage;
        }

        let chance = self.get_observed_crit_chance(weapon);
        log!("my chance {}",chance);
        log!("{:?}", self.damage_till_crit(weapon));
        dbg!(self.crit_damage);
        dbg!(self.melee_damage);
    }
    pub fn player_spawn(&mut self, event: &GameEvent) {
        if !self.crit_cmd_numbers.is_empty() {
            return;
        }
        let user_id = event.get_int("userid").unwrap();
        let p_local = Player::get_local().unwrap();
        if user_id != p_local.info().unwrap().user_id {
            return;
        }

        for weapon in p_local.get_weapons() {
            let cmds = self.find_crit_cmds(0, p_local, weapon);
            let id = *weapon.get_item_definition_index();
            self.crit_cmd_numbers.insert(id, cmds);
        }
    }
    pub fn find_crit_cmds(
        &mut self,
        curr_cmd_number: i32,
        owner: &mut Player,
        weapon: &mut Weapon,
    ) -> Vec<i32> {
        let weapon_index = vmt_call!(weapon.as_ent(), get_index);
        let owner_index = vmt_call!(owner.as_ent(), get_index);
        let index_seed_mask = (weapon_index << 8) | owner_index;

        let crit_chance = owner.get_crit_mult() * TF_DAMAGE_CRIT_CHANCE;

        let mut cmds = Vec::new();
        for cmd_number in curr_cmd_number..curr_cmd_number + 10000 {
            let seed = ((o!().util.md5_pseudorandom)(cmd_number) & 0x7FFFFFFF) ^ index_seed_mask;
            (o!().util.random_seed)(seed);
            let random = (o!().util.random_int)(0, WEAPON_RANDOM_RANGE - 1);
            if (random as f32) < crit_chance * WEAPON_RANDOM_RANGE as f32 {
                cmds.push(cmd_number);
            }
        }
        cmds
    }
    pub fn damage_till_crit(&mut self, weapon: &mut Weapon) -> Option<f32> {
        let p_local = Player::get_local().unwrap();
        let crit_chance = p_local.get_crit_mult() * TF_DAMAGE_CRIT_CHANCE;

        let curr_crit = self.crit_cap(weapon);
        let observed_chance = *weapon.get_observed_crit_chance();
        let needed_chance = curr_crit + 0.1;
        dbg!(observed_chance, needed_chance);
        if observed_chance <= needed_chance {
            return None;
        }
        //int damage = std::ceil(crit_damage * (2.0f * target_chance + 1.0f) / (3.0f * target_chance));

        Some(self.crit_damage * (2.0 * needed_chance + 1.0) / (3.0 * needed_chance))
    }
    pub fn crit_cap(&mut self, weapon: &mut Weapon) -> f32 {
        if let Some(val) = weapon.as_ent().get_float_attrib("mult_crit_chance") {
            val
        } else {
            let p_local = Player::get_local().unwrap();
            let crit_mutl = p_local.get_crit_mult();
            let chance = 0.2;
            chance * crit_mutl
        }
    }
    pub fn get_observed_crit_chance(&self, weapon: &mut Weapon) -> f32{
        let p_local = Player::get_local().unwrap();
        let resource = p_local.get_resource_data();

        let cached_damage = resource.damage as f32 - self.melee_damage;
        if cached_damage - resource.damage  as f32 == 0.0 {
            return 0.0;
        }

        let normalized_crit_damage = self.crit_damage/3.0;
        dbg!(normalized_crit_damage,cached_damage,resource.damage,self.crit_damage);

        normalized_crit_damage / ((cached_damage - resource.damage as f32) - self.crit_damage)
        //dbg!(resource.damage);
        //    if (!(cached_damage - round_damage))
        //        return 0.0f;
        //    // Same is used by server
        //    float normalized_damage = (float) crit_damage / 3.0f;
        //    return normalized_damage / (normalized_damage + (float) ((cached_damage - round_damage) - crit_damage));
    }
}

impl Cheat for CritManipulation {}
