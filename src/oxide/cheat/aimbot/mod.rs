use crate::{
    draw::event::EventType,
    error::OxideResult,
    math::vector3::Vector3,
    sdk::{
        condition::ConditionFlags,
        entity::{
            player::Player,
            weapon::ids::{WeaponId, WeaponType},
            Entity,
        },
        interfaces::{
            engine_trace::{trace, CONTENTS_GRATE, MASK_SHOT},
            entity::hitbox::{HitboxId, HitboxWrapper},
        },
        user_cmd::{ButtonFlags, UserCmd},
    },
    setting, vmt_call,
};

use self::priority::Priority;

use super::Cheat;

pub mod player;
pub mod priority;
pub mod sentry;
pub mod sticky;

#[derive(Debug, Clone)]
pub struct Aimbot {
    pub shoot_key_pressed: bool,
}
#[derive(Debug, Clone)]
pub struct Target {
    pub point: Vector3,
    pub ent: &'static Entity,
    pub hitbox_id: HitboxId,
    pub prio: Priority,
}

impl Aimbot {
    pub fn name() -> &'static str {
        "Aimbot"
    }
    pub fn init() -> Aimbot {
        Aimbot {
            shoot_key_pressed: false,
        }
    }

    pub fn point_scan(&self, hitbox: &HitboxWrapper) -> OxideResult<Option<(Vector3, isize)>> {
        let p_local = Player::get_local().unwrap();
        let my_eyes = vmt_call!(p_local.as_ent(), eye_position);

        let mut scaled_hitbox = hitbox.scaled(setting!(aimbot, hitbox_scale));

        let mut points = vec![scaled_hitbox.center()?];
        if setting!(aimbot, multipoint) {
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
            let trace = trace(my_eyes.clone(), point.clone(), MASK_SHOT | CONTENTS_GRATE);

            if trace.entity != hitbox.owner
                || (hitbox.id == HitboxId::Head && trace.hitbox_id != hitbox.id)
            {
                continue;
            }
            return Ok(Some((point, prio)));
        }
        Ok(None)
    }

    pub fn find_target(&self) -> OxideResult<Option<Target>> {
        let mut target = self.find_player()?;

        if setting!(aimbot, target_sentries) {
            if let Some(sentry) = self.find_sentry()? {
                if let Some(target) = &mut target {
                    if sentry.prio > target.prio {
                        *target = sentry;
                    }
                } else {
                    target = Some(sentry);
                };
            }
        }

        if setting!(aimbot, target_stickies) && target.is_none() {
            target = self.find_sticky()?;
        }
        Ok(target)
    }

    pub fn should_run(&self) -> bool {
        let p_local = Player::get_local().unwrap();
        if !setting!(aimbot, enabled) || (!self.shoot_key_pressed && !setting!(aimbot, always_on)) {
            return false;
        }

        if !vmt_call!(p_local.as_ent(), is_alive) {
            return false;
        }

        true
    }

    pub fn create_move(&mut self, cmd: &mut UserCmd) -> OxideResult<Option<Target>> {
        let mut target = None;
        if !self.should_run() {
            return Ok(target);
        }
        let p_local = Player::get_local().unwrap();
        let weapon = vmt_call!(p_local.as_ent(), get_weapon);
        if weapon.as_gun().is_ok() {
            target = if setting!(aimbot, aim_while_on_delays) || p_local.can_attack() {
                self.find_target()?
            } else {
                None
            };

            if let Some(target) = &target {
                let my_eyes = vmt_call!(p_local.as_ent(), eye_position);
                let diff = target.point - my_eyes;

                let angle = diff.angle();
                if setting!(aimbot, autoshoot) {
                    if self.shoot_weapon(cmd, Some(target)) {
                        cmd.viewangles = angle.to_view_angles();
                    }
                } else {
                    cmd.viewangles = angle.to_view_angles();
                }
            } else {
                self.shoot_weapon(cmd, None);
            }
        }

        Ok(target)
    }
    pub fn shoot_weapon(&mut self, cmd: &mut UserCmd, found: Option<&Target>) -> bool {
        let p_local = Player::get_local().unwrap();
        let weapon = vmt_call!(p_local.as_ent(), get_weapon);
        let id = vmt_call!(weapon, get_weapon_id);

        let Some(found) = found else {
            if  weapon.is_sniper_rifle() && setting!(aimbot, auto_unscope) &&  p_local.get_condition().get(ConditionFlags::Zoomed) {
                cmd.buttons.set(ButtonFlags::InAttack2, true);
                return false;

            }
            return false;
        };

        if weapon.is_sniper_rifle() {
            let classic = matches!(id, WeaponType::SniperrifleClassic);
            if setting!(aimbot, auto_scope) {
                let lethal = weapon.as_gun().unwrap().is_lethal(found.ent, false);
                if !p_local.get_condition().get(ConditionFlags::Zoomed) && !classic && !lethal {
                    cmd.buttons.set(ButtonFlags::InAttack2, true);
                    return false;
                }
                if !vmt_call!(weapon, can_fire_critical_shot, true)
                    && !matches!(
                        weapon.get_item_definition_index(),
                        WeaponId::SniperMTheSydneySleeper
                    )
                {
                    return false;
                }
            }
            cmd.buttons.set(ButtonFlags::InAttack, !classic);
            return true;
        }
        match id {
            WeaponType::Knife => {
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
        let aimbot_key = setting!(aimbot, key);
        match event.r#type {
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
            _ => (),
        }
    }
}
