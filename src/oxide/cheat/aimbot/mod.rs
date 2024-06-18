use std::{time::Instant, usize};

use crate::{
    draw::{component::base::key_input::KeyInputValue, event::EventType},
    error::OxideResult,
    math::vector3::Vector3,
    sdk::{
        condition::ConditionFlags,
        entity::{
            player::Player,
            weapon::ids::{ItemDefinitionInex, WeaponId},
            Entity,
        },
        interfaces::{
            engine_trace::{trace, CONTENTS_GRATE, MASK_SHOT},
            entity::hitbox::{HitboxWrapper, PlayerHitboxId},
        },
        user_cmd::{ButtonFlags, UserCmd},
    },
    setting, vmt_call,
};

use self::priority::Priority;

use super::Cheat;

pub mod object;
pub mod player;
pub mod priority;
pub mod sticky;

#[derive(Debug, Clone)]
pub struct Aimbot {
    pub shoot_key_pressed: bool,
    pub last_target: Option<(Target, Instant)>,
}
#[derive(Debug, Clone)]
pub struct Target {
    pub point: Vector3,
    pub ent: i32,
    pub hitbox_id: usize,
    pub prio: Priority,
}

impl Aimbot {
    pub fn init() -> Aimbot {
        Aimbot {
            shoot_key_pressed: false,
            last_target: None,
        }
    }

    pub fn trace_point(&self, point: Vector3, hitbox: &HitboxWrapper) -> bool {
        let p_local = Player::get_local().unwrap();
        let my_eyes = vmt_call!(p_local.as_ent(), eye_position);
        let trace = trace(my_eyes.clone(), point.clone(), MASK_SHOT | CONTENTS_GRATE);

        trace.entity == hitbox.owner
            && (hitbox.id != PlayerHitboxId::Head as usize || trace.hitbox_id == hitbox.id)
    }

    pub fn point_scan(&self, hitbox: &HitboxWrapper) -> OxideResult<Option<(Vector3, isize)>> {
        let p_local = Player::get_local().unwrap();
        let my_eyes = vmt_call!(p_local.as_ent(), eye_position);

        let mut scaled_hitbox = hitbox.scaled(*setting!(aimbot, hitbox_scale));

        let mut points = vec![scaled_hitbox.center()?];
        if *setting!(aimbot, multipoint) {
            let mut corners = scaled_hitbox.corners()?.to_vec();

            corners.sort_by(|corner1, corner2| {
                let diff1 = corner1.clone() - my_eyes.clone();
                let diff2 = corner2.clone() - my_eyes.clone();
                diff1.len().total_cmp(&diff2.len())
            });

            corners.pop();
            corners.remove(0);

            points = vec![points, corners].concat();
        }

        for point in points {
            let Some(prio) = self.point_priority(point.clone()) else {
                continue;
            };
            if !self.trace_point(point, hitbox) {
                continue;
            }
            return Ok(Some((point, prio)));
        }
        Ok(None)
    }

    pub fn find_targets(&mut self) -> OxideResult<Option<Target>> {
        let mut target: Option<Target> = None;
        if let Some(last_target) = &self.last_target {
            if Instant::now().duration_since(last_target.1).as_secs_f32()
                <= *setting!(aimbot, target_persistance_duration)
            {
                if let Some(ent) = Entity::get_ent(last_target.0.ent) {
                    if ent.as_player().is_ok() {
                        self.scan_player_hitboxes(last_target.0.ent, &mut target)?;
                    } else if ent.as_object().is_ok() {
                        self.scan_object_hitboxes(last_target.0.ent, &mut target)?;
                    } else if ent.as_pipe().is_ok() {
                        self.scan_sticky_hitboxes(last_target.0.ent, &mut target)?;
                    }
                }
            }
        }
        if target.is_none() {
            self.find_valid_player_target(&mut target)?;
            self.find_object(&mut target)?;
            self.find_sticky(&mut target)?;
            self.last_target = target.clone().map(|x| (x, Instant::now()));
        }
        Ok(target)
    }

    pub fn should_run(&self) -> bool {
        let Ok(p_local) = Player::get_local() else {return false};
        if !*setting!(aimbot, enabled) || (!self.shoot_key_pressed && !*setting!(aimbot, always_on))
        {
            return false;
        }

        if !vmt_call!(p_local.as_ent(), is_alive) {
            return false;
        }

        true
    }

    pub fn create_move(&mut self, cmd: &mut UserCmd) -> OxideResult<Option<Target>> {
        if !self.should_run() {
            self.last_target = None;
            return Ok(None);
        }
        let p_local = Player::get_local().unwrap();
        let weapon = vmt_call!(p_local.as_ent(), get_weapon);
        if weapon.as_gun().is_err() {
            self.last_target = None;
            return Ok(None);
        }
        let mut target = None;
        if !*setting!(aimbot, fire_only_when_able) || p_local.can_attack() {
            target = self.find_targets()?;
        }

        if let Some(target) = &target {
            let my_eyes = vmt_call!(p_local.as_ent(), eye_position);
            let diff = target.point - my_eyes;

            let angle = diff.angle();
            if *setting!(aimbot, autoshoot) {
                if self.shoot_weapon(cmd, Some(target)) {
                    cmd.viewangles = angle;
                }
            } else {
                cmd.viewangles = angle;
            }
        } else {
            self.shoot_weapon(cmd, None);
        }

        Ok(target)
    }
    pub fn shoot_weapon(&mut self, cmd: &mut UserCmd, found: Option<&Target>) -> bool {
        let p_local = Player::get_local().unwrap();
        let weapon = vmt_call!(p_local.as_ent(), get_weapon);
        let id = vmt_call!(weapon, get_weapon_id);

        if found.is_none() {
            if weapon.is_sniper_rifle()
                && *setting!(aimbot, auto_unscope)
                && p_local.get_condition().get(ConditionFlags::Zoomed)
            {
                cmd.buttons.set(ButtonFlags::InAttack2, true);
            }
            if *setting!(aimbot, auto_rev) && weapon.is_minigun() {
                cmd.buttons.set(ButtonFlags::InAttack2, true);
                return false;
            }
            return false;
        };

        if weapon.is_sniper_rifle() {
            let classic = matches!(id, WeaponId::SniperrifleClassic);
            if *setting!(aimbot, auto_scope) {
                if !p_local.get_condition().get(ConditionFlags::Zoomed) && !classic {
                    cmd.buttons.set(ButtonFlags::InAttack2, true);
                    return false;
                }
                if !vmt_call!(weapon, can_fire_critical_shot, true)
                    && !matches!(
                        weapon.get_item_definition_index(),
                        ItemDefinitionInex::SniperMTheSydneySleeper
                    )
                {
                    return false;
                }
            }
            cmd.buttons.set(ButtonFlags::InAttack, !classic);
            return true;
        }
        match id {
            WeaponId::Knife => {
                if weapon.as_mele().ready_to_backstab {
                    cmd.buttons.set(ButtonFlags::InAttack, true);
                    return true;
                }
                false
            }
            _ => {
                cmd.buttons.set(ButtonFlags::InAttack, true);
                true
            }
        }
    }
}

impl Cheat for Aimbot {
    fn handle_event(&mut self, event: &mut crate::draw::event::Event) {
        let aimbot_key = *setting!(aimbot, key);
        match aimbot_key {
            KeyInputValue::Keyboard(aimbot_key) => match event.r#type {
                EventType::KeyDown(key) => {
                    if key == *aimbot_key {
                        self.shoot_key_pressed = true;
                        event.handled = true;
                    }
                }
                EventType::KeyUp(key) => {
                    if key == *aimbot_key {
                        self.shoot_key_pressed = false;
                        event.handled = true;
                    }
                }
                _ => {}
            },
            KeyInputValue::Mouse(aimbot_key) => match event.r#type {
                EventType::MouseButtonUp(key) => {
                    if key == aimbot_key {
                        self.shoot_key_pressed = false;
                        event.handled = true;
                    }
                }
                EventType::MouseButtonDown(key) => {
                    if key == aimbot_key {
                        self.shoot_key_pressed = true;
                        event.handled = true;
                    }
                }
                _ => (),
            },
        }
    }
}
