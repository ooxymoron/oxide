use std::mem::transmute;

use crate::{
    draw::event::Event, log, o, sdk::{
        entity::{player::Player, weapon::Weapon},
        game_event::GameEvent,
        user_cmd::{ButtonFlags, UserCmd},
    }, setting
};

use super::Cheat;
pub mod seed_manipulation;
pub mod crit_chance_correction;

pub const BUCKET_CAP: f32 = 1000.0;
pub const WEAPON_RANDOM_RANGE: i32 = 10000;

pub const CRIT_CHANCE: f32 = 0.02;
pub const CRIT_CHANCE_MELEE: f32 = 0.15;
pub const CRIT_DURATION_RAPID: f32 = 2.0;
pub const CRIT_MULTIPLIER: f32 = 3.0;

const CMD_SCAN_LIMIT: i32 = 10000;
#[derive(Debug)]
pub struct CritManipulationState {
    pub needed_damage: Option<f32>,
    pub needed_blanks: Option<i32>,
    pub crits: i32,
    pub max_crits: i32,
    pub next_check: Option<f32>,
    pub crit_time: Option<f32>,
}

#[derive(Debug)]
pub struct CritManipulation {
    crit_damage: f32,
    melee_damage: f32,
    pub crit_key_pressed: bool,
    last_spoofed_cmd_num: i32,
    pub last_spoofed_cmd_seed: Option<i32>,
    pub state: Option<CritManipulationState>,
}

impl CritManipulation {
    pub fn init() -> CritManipulation {
        CritManipulation {
            crit_damage: 0.0,
            melee_damage: 0.0,
            crit_key_pressed: false,
            last_spoofed_cmd_num: 0,
            last_spoofed_cmd_seed: None,
            state: None,
        }
    }
    pub fn reset(&mut self) {
        self.crit_damage = 0.0;
        self.melee_damage = 0.0;
        self.last_spoofed_cmd_num = 0;
        self.last_spoofed_cmd_seed = None;
    }
    pub fn damage_till_crit(&mut self, weapon: &mut Weapon) -> Option<f32> {
        let owner = weapon.get_owner().resolve().unwrap().as_player().unwrap();
        let crit_chance = self.calc_crit_chance(weapon);

        let observed_chance = self.calc_observed_crit_chance();


        if observed_chance <= crit_chance + 0.1 {
            return None;
        }

        let needed_damage = 
            (
                ((2./3.) * self.crit_damage) * 
                  ((3./5.) - crit_chance )
            )
            /
            (crit_chance + 1./10.)
        ;

        let resource = owner.get_resource_data();
        let ranged_damage = resource.damage as f32 - self.melee_damage;
        let diff = needed_damage - ranged_damage;
        if diff > 0.0 {
            return Some(diff);
        }
        //should never happen
        Some(-1.)
    }
    pub fn blanks_till_crit(&mut self, weapon: &mut Weapon) -> Option<i32> {
        if weapon.get_info().melee_weapon {
            return None
        }

        let data = weapon.get_info().weapon_data[weapon.get_mode()].clone();
        let mut weapon_damage = data.damage;
        if let Ok(gun) = weapon.as_gun() {
            weapon_damage *= gun.get_bullets()
        }
        let mut damage = weapon_damage as f32;

        if data.use_rapid_fire_crits {
            damage *= CRIT_DURATION_RAPID / data.time_fire_delay;
        }
        if damage * CRIT_MULTIPLIER > BUCKET_CAP {
            damage = BUCKET_CAP as f32 / CRIT_MULTIPLIER
        }
        let bucket = *weapon.get_crit_bucket();
        let requests = *weapon.get_crit_seed_requests() as f32 + 1.;
        let checks = *weapon.get_crit_checks();
         if bucket != BUCKET_CAP {
            return None
        }
         let needed_checks = (
            requests/(150./damage - 7./20.)
        ).ceil() as i32;


        if needed_checks > checks{
            return Some(needed_checks- checks)
        }

        None
    }
    pub fn calc_crit_chance(&self, weapon: &mut Weapon) -> f32{
        let owner = weapon.get_owner().resolve().unwrap().as_player().unwrap();
        let data = weapon.get_info().weapon_data[weapon.get_mode()].clone();

        if let Some(attrib_crit_chance) = weapon.as_ent().get_float_attrib("mult_crit_chance") {
            attrib_crit_chance
        }else if data.use_rapid_fire_crits {
            1.0 / (
                (
                    CRIT_DURATION_RAPID
                    / 
                    (owner.get_crit_mult() * CRIT_CHANCE).clamp(0.01, 0.99)
                )
                - CRIT_DURATION_RAPID
            )
        } else {
            owner.get_crit_mult() * (if weapon.get_info().melee_weapon {CRIT_CHANCE_MELEE} else {CRIT_CHANCE})
        }
    }
    pub fn update_state(&mut self, weapon: &mut Weapon) {
        let crits = weapon.crits();
        self.state = Some(CritManipulationState {
            needed_damage: self.damage_till_crit(weapon),
            needed_blanks: self.blanks_till_crit(weapon),
            crits: crits.0,
            max_crits: crits.1,
            next_check: weapon.get_next_check(),
            crit_time: weapon.get_crit_time(),
        })
    }
}

impl CritManipulation {
    pub fn player_hurt(&mut self, event: &GameEvent) {
        let Ok(p_local) = Player::get_local() else {return};
        let Ok(attacker) = Player::get_from_user_id(event.get_int("attacker").unwrap()) else {return};
        if attacker.as_ent() != p_local.as_ent() {
            return;
        }
        let attacked = Player::get_from_user_id(event.get_int("userid").unwrap()).unwrap();
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
        if weapon.get_info().melee_weapon {
            self.melee_damage += damage;
        } else if crit {
            self.crit_damage += damage;
        }
    }
    pub fn create_move(&mut self, cmd: &mut UserCmd) {
        if o!().player_resource_manager.entity.is_none() {
            return
        }
        let Ok(p_local) = Player::get_local() else { 
            self.state = None;
            return
        };
        let weapon = p_local.weapon();
        if !weapon.can_crit() {
            self.state = None;
            return
        }
        self.update_state(weapon);

        if  !p_local.can_attack() {
            return;
        }
        let blanks_till_crit = self.blanks_till_crit(weapon);
        let info = &weapon.get_info().weapon_data[weapon.get_mode()];

        if  
               !cmd.buttons.get(ButtonFlags::InAttack) 
            && blanks_till_crit.is_some() 
            && self.crit_key_pressed 
            && *setting!(crit_manipulation,auto_cycle_rapid_fire) 
            && info.use_rapid_fire_crits
            && weapon.get_next_check().is_none()
            && weapon.get_crit_time().is_none()
        {
            cmd.buttons.set(ButtonFlags::InAttack, true)
        }

        if !cmd.buttons.get(ButtonFlags::InAttack)  {
            return;
        }

        let damage_till_crit = self.damage_till_crit(weapon);
        let crits = weapon.crits();
        let crit = if damage_till_crit.is_some()
            || crits.0 == 0
            || !self.crit_key_pressed
            || weapon.get_next_check().is_some()
            || weapon.get_crit_time().is_some()
            || blanks_till_crit.is_some()
        {
            false
        } else {
            true
        };
        let (cmd_num, seed) = self
            .my_find_cmd(
                cmd.command_number.max(self.last_spoofed_cmd_num + 1),
                weapon,
                crit,
            )
            .unwrap();

        self.last_spoofed_cmd_num = cmd_num;
        self.last_spoofed_cmd_seed = Some(seed);
        cmd.command_number = cmd_num;
        cmd.seed = seed;
        log!(
            "{} | {} / {}",
            weapon.get_crit_bucket().clone(),
            weapon.get_crit_seed_requests().clone(),
            weapon.get_crit_checks(),
        );
    }
}

impl Cheat for CritManipulation {
    fn handle_event(&mut self, event: &mut Event) {
        let crit_key = setting!(crit_manipulation, key);
        if event.is_key_down(&crit_key) {
            self.crit_key_pressed = true
        }
        if event.is_key_up(&crit_key) {
            self.crit_key_pressed = false
        }
    }
}
