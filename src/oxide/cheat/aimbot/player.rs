use crate::{
    error::OxideResult,
    o,
    sdk::{
        condition::ConditionFlags,
        entity::{
            hitbox::{HitboxId, HitboxWrapper},
            player::Player,
            Entity,
        },
        networkable::ClassId,
    },
    setting, vmt_call,
};

use super::{priority::Priority, Aimbot, Target};

impl<'player> Aimbot {
    pub fn player_hitbox_order(
        &self,
        player: &'player Player,
    ) -> Vec<(isize, &'player HitboxWrapper)> {
        let p_local = Player::get_local().unwrap();
        let weapon = vmt_call!(p_local.as_ent(), get_weapon);
        let baim = (|| {
            if weapon.can_headshot() {
                if weapon.is_sniper_rifle() && !p_local.get_condition().get(ConditionFlags::Zoomed)
                {
                    return true;
                }
                return setting!(aimbot, baim_if_lethal)
                    && weapon.as_gun().unwrap().is_lethal(player.as_ent(), false);
            }
            return true;
        })();

        let hitboxes = player.as_ent().get_hitboxes().unwrap();
        if baim {
            if weapon.is_sniper_rifle()
                && setting!(aimbot, wait_for_charge)
                && p_local.get_condition().get(ConditionFlags::Zoomed)
                && !weapon.as_gun().unwrap().is_lethal(player.as_ent(), false)
            {
                return vec![];
            }
            hitboxes
                .values()
                .map(|hitbox| {
                    if matches!(hitbox.id, HitboxId::Pelvis) {
                        (1, hitbox)
                    } else {
                        (0, hitbox)
                    }
                })
                .collect()
        } else {
            if weapon.is_sniper_rifle()
                && setting!(aimbot, wait_for_charge)
                && p_local.get_condition().get(ConditionFlags::Zoomed)
                && !weapon.as_gun().unwrap().is_lethal(player.as_ent(), true)
            {
                return vec![];
            }
            hitboxes
                .values()
                .map(|hitbox| {
                    if matches!(hitbox.id, HitboxId::Head) {
                        (1, hitbox)
                    } else {
                        (0, hitbox)
                    }
                })
                .collect()
        }
    }
    pub fn player_prioroty(&self, player: &Player) -> OxideResult<Option<isize>> {
        if self.ent_priority(player.as_ent())?.is_none() {
            return Ok(None);
        }
        let p_local = Player::get_local().unwrap();
        let weapon = vmt_call!(p_local.as_ent(), get_weapon);

        if weapon.is_ambassador()
            && setting!(aimbot, ambasador_wait_for_hs)
            && o!().global_vars.curtime - *weapon.get_last_fire() < 1.0
            && !weapon.as_gun().unwrap().is_lethal(player.as_ent(), false)
        {
            return Ok(None);
        }
        let mut ignore_flags = vec![ConditionFlags::Ubercharged, ConditionFlags::Bonked];
        let spy_revealing_flags = vec![
            ConditionFlags::Jarated,
            ConditionFlags::Milked,
            ConditionFlags::CloakFlicker,
            ConditionFlags::OnFire,
            ConditionFlags::Bleeding,
        ];
        let conditions = player.get_condition();

        if spy_revealing_flags
            .into_iter()
            .all(|flag| !conditions.get(flag))
        {
            if !setting!(aimbot, target_invisible) {
                ignore_flags.push(ConditionFlags::Cloaked)
            }
            if !setting!(aimbot, target_disguised) {
                ignore_flags.push(ConditionFlags::Disguised)
            }
        }

        if ignore_flags.into_iter().any(|flag| conditions.get(flag)) {
            return Ok(None);
        }
        Ok(Some(1))
    }

    pub fn find_player(&self) -> OxideResult<Option<Target>> {
        let mut best_target: Option<Target> = None;
        for id in o!()
            .last_entity_cache
            .as_ref()
            .unwrap()
            .get_ent(ClassId::CTFPlayer)
        {
            let Some(player) = Entity::get_ent(id) else {continue};
            if vmt_call!(player.as_networkable(), is_dormant) || !vmt_call!(player, is_alive) {
                continue;
            }

            let Some(player_prioroty) = self.player_prioroty(player.as_player()?)? else {
                continue;
            };
            if let Some(best_target) = &best_target {
                if best_target.prio.ent > player_prioroty {
                    continue;
                }
            }

            for (hitbox_prio, hitbox) in self.player_hitbox_order(player.as_player()?) {
                if let Some(target) = &best_target {
                    if target.prio.hitbox > hitbox_prio {
                        break;
                    }
                }

                let Some((point,point_prio)) = self.point_scan(&hitbox)? else {
                    continue;
                };

                if let Some(target) = &best_target {
                    if target.prio.point > point_prio {
                        continue;
                    }
                }
                let prio = Priority {
                    ent: player_prioroty,
                    hitbox: hitbox_prio,
                    point: point_prio,
                };
                let target = Target {
                    point,
                    ent: player,
                    hitbox_id: hitbox.id,
                    prio,
                };
                best_target = Some(target);
            }
        }
        Ok(best_target)
    }
}
