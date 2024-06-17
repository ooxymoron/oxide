use crate::{
    error::OxideResult,
    o,
    sdk::{
        condition::ConditionFlags,
        entity::{
            hitbox::{HitboxWrapper, PlayerHitboxId},
            player::Player,
            Entity,
        },
        networkable::ClassId,
    },
    setting, vmt_call,
};

use super::{priority::Priority, Aimbot, Target};

#[derive(Debug, Clone, Copy)]
pub enum HitboxPriority {
    HeadOnly,
    PrioHead,
    BodyOnly,
    All,
}

impl<'player> Aimbot {
    pub fn player_hitbox_order(
        &self,
        player: &'player Player,
        prio: HitboxPriority,
    ) -> Vec<(isize, &'player HitboxWrapper)> {
        let hitboxes = player.as_ent().get_hitboxes().unwrap();
        let target_hitboxes = setting!(aimbot, hitboxes);
        hitboxes
            .values()
            .filter_map(|hitbox| {
                let hitbox_id = PlayerHitboxId::from(hitbox.id);
                if matches!(prio, HitboxPriority::HeadOnly)
                    && !matches!(hitbox_id, PlayerHitboxId::Head)
                {
                    return None;
                };
                match hitbox_id {
                    PlayerHitboxId::Head => match prio {
                        HitboxPriority::HeadOnly | HitboxPriority::PrioHead => Some((3, hitbox)),
                        HitboxPriority::BodyOnly => None,
                        HitboxPriority::All => Some((1, hitbox)),
                    },
                    PlayerHitboxId::Pelvis
                    | PlayerHitboxId::Spine0
                    | PlayerHitboxId::Spine1
                    | PlayerHitboxId::Spine2
                    | PlayerHitboxId::Spine3 => Some((2, hitbox)),
                    PlayerHitboxId::LeftHip
                    | PlayerHitboxId::RightHip
                    | PlayerHitboxId::LeftKnee
                    | PlayerHitboxId::RightKnee => Some((1, hitbox)),
                    _ => Some((0, hitbox)),
                }
            })
            .filter(|(_, hitbox)| target_hitboxes.contains(&PlayerHitboxId::from(hitbox.id)))
            .collect()
    }
    pub fn player_prioroty(&self, player: &Player) -> OxideResult<Option<isize>> {
        if self.ent_priority(player.as_ent())?.is_none() {
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
            if !*setting!(aimbot, target_invisible) {
                ignore_flags.push(ConditionFlags::Cloaked)
            }
            if !*setting!(aimbot, target_disguised) {
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

        let p_local = Player::get_local().unwrap();
        let weapon = p_local.weapon();
        for id in o!()
            .last_entity_cache
            .as_ref()
            .unwrap()
            .get_class_ids(ClassId::CTFPlayer)
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

            let hp = vmt_call!(player, get_health) as f32;

            let mut hitbox_order_prio = if weapon.can_headshot() {
                HitboxPriority::PrioHead
            } else {
                HitboxPriority::All
            };
            if matches!(hitbox_order_prio, HitboxPriority::PrioHead) {
                if let Ok(gun) = weapon.as_gun() {
                    if *setting!(aimbot, wait_for_charge) {
                        if hp > gun.get_damage(true) {
                            continue;
                        }
                        if hp > gun.get_damage(false) {
                            hitbox_order_prio = HitboxPriority::HeadOnly;
                        }
                    }
                    if gun.get_damage(false) >= hp && *setting!(aimbot, baim_if_lethal) {
                        hitbox_order_prio = HitboxPriority::BodyOnly;
                    }
                }
            }
            for (hitbox_prio, hitbox) in
                self.player_hitbox_order(player.as_player()?, hitbox_order_prio)
            {
                if let Some(target) = &best_target {
                    if target.prio.hitbox > hitbox_prio {
                        continue;
                    }
                }

                let Some((point,point_prio)) = self.point_scan(&hitbox)? else {
                    continue;
                };

                if let Some(target) = &best_target {
                    if target.prio.point > point_prio && target.prio.hitbox == hitbox_prio {
                        continue;
                    }
                }
                let target = Target {
                    point,
                    ent: player,
                    hitbox_id: hitbox.id,
                    prio: Priority {
                        ent: player_prioroty,
                        hitbox: hitbox_prio,
                        point: point_prio,
                    },
                };
                best_target = Some(target);
            }
        }
        Ok(best_target)
    }
}
