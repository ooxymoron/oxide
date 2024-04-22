use crate::{
    draw::event::EventType,
    error::OxideResult,
    math::vector::Vector3,
    o,
    sdk::{
        condition::ConditionFlags,
        engine_trace::{trace, CONTENTS_GRATE, MASK_SHOT},
        entity::{weapon::ids::{ItemDefiniitonIndex, WeaponType}, Entity},
        model_info::{Hitbox, HitboxId},
        networkable::ClassId,
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

    pub fn point_scan(
        &self,
        ent: &Entity,
        hitboxid: HitboxId,
        hitbox: &Hitbox,
    ) -> OxideResult<Option<(Vector3, isize)>> {
        let p_local = &*Entity::get_local().unwrap();
        let my_eyes = vmt_call!(p_local.as_ent(), eye_position);

        let scaled_hitbox = hitbox.scaled(setting!(aimbot, hitbox_scale));

        let mut points = vec![scaled_hitbox.center(&ent)?];
        if setting!(aimbot, multipoint) {
            let mut corners = scaled_hitbox.corners(&ent)?.to_vec();

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
            if trace.entity as *const _ != ent
                || (trace.hitbox != hitboxid as HitboxId && hitboxid == HitboxId::Head)
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
        let p_local = Entity::get_local().unwrap();
        if !setting!(aimbot, enabled) || !self.shoot_key_pressed {
            return false;
        }

        if !vmt_call!(p_local.as_ent(), is_alive) {
            return false;
        }

        true
    }

    pub fn create_move(&mut self, cmd: &mut UserCmd) -> OxideResult<()> {
        if !self.should_run() {
            return Ok(());
        }

        if let Some(target) = self.find_target()? {
            let p_local = &*Entity::get_local().unwrap();
            let my_eyes = vmt_call!(p_local.as_ent(), eye_position);
            let diff = my_eyes - target.point;
            let angle = diff.angle();
            if setting!(aimbot, autoshoot) {
                if self.shoot(cmd, Some(target)) {
                    cmd.viewangles = angle;
                }
            } else {
                cmd.viewangles = angle;
            }
        } else {
            self.shoot(cmd, None);
        }
        Ok(())
    }
    pub fn shoot(&mut self, cmd: &mut UserCmd, found: Option<Target>) -> bool {
        let p_local = &*Entity::get_local().unwrap();
        let weapon = vmt_call!(p_local.as_ent(), get_weapon);
        let id = vmt_call!(weapon, get_weapon_id);

        //let attributes = weapon.as_ent().attributes;
        //let manager = vmt_call!(attributes, get_attribute_manager);
        //let res = (*manager).get_float("bodyshot_damage_modify", weapon.as_ent(), 1.0);
        //let res = (*manager).get_float("mult_bullets_per_shot", weapon.as_ent(), 1.0);
        let Some(found) = found else {
            if matches!(id, WeaponType::SniperrifleClassic) {
                cmd.buttons.set(ButtonFlags::InAttack, true);
            }
            return false;
        };
        let class = found.ent.as_networkable().get_client_class().class_id;
        if matches!(
            weapon.get_item_definition_index(),
            ItemDefiniitonIndex::SpyMTheAmbassador | ItemDefiniitonIndex::SpyMFestiveAmbassador
        ) && setting!(aimbot, ambasador_wait_for_hs)
        {
            let baim_lethal = (setting!(aimbot, baim_if_lethal)
                && vmt_call!(weapon.as_gun(), get_projectile_damage)
                    >= (vmt_call!(found.ent, get_health)) as f32);
            let shoot = o!().global_vars.curtime - *weapon.get_last_fire() > 1.0
                || baim_lethal
                || matches!(class, ClassId::CObjectSentrygun);

            if shoot {
                cmd.buttons.set(ButtonFlags::InAttack, shoot);
            }
            return shoot;
        }

        match id {
            WeaponType::Sniperrifle
            | WeaponType::SniperrifleDecap
            | WeaponType::SniperrifleClassic => {
                let baim_lethal = (setting!(aimbot, baim_if_lethal)
                    && vmt_call!(weapon.as_gun(), get_projectile_damage)
                        >= (vmt_call!(found.ent, get_health)) as f32);
                let classic = matches!(id, WeaponType::SniperrifleClassic);
                if !p_local.get_condition().get(ConditionFlags::Zoomed) && !classic && !baim_lethal {
                    cmd.buttons.set(ButtonFlags::InAttack2, true);
                    return false;
                }
                if !p_local.can_attack()
                    || (!vmt_call!(weapon, can_fire_critical_shot, true) && !baim_lethal)
                {
                    if classic {
                        cmd.buttons.set(ButtonFlags::InAttack, true);
                    }
                    return false;
                }
                cmd.buttons.set(ButtonFlags::InAttack, !classic);
                true
            }
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
