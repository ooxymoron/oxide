use crate::{
    error::OxideResult,
    o,
    sdk::{
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
                if !target_hitboxes.contains(&PlayerHitboxId::from(hitbox_id)) {
                    return None;
                }
                if matches!(prio, HitboxPriority::HeadOnly)
                    && !matches!(hitbox_id, PlayerHitboxId::Head)
                {
                    return None;
                }
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
            .collect()
    }

    pub fn scan_player_hitboxes(
        &self,
        id: i32,
        best_target: &mut Option<Target>,
    ) -> OxideResult<()> {
        let Some(player) = Entity::get_ent(id) else {return Ok(())};
        if vmt_call!(player.as_networkable(), is_dormant) || !vmt_call!(player, is_alive) {
            return Ok(());
        }

        let Some(player_prioroty) = player.priority() else {
            return Ok(());
        };
        if let Some(best_target) = &best_target {
            if best_target.prio.ent > player_prioroty {
                return Ok(());
            }
        }

        let hp = vmt_call!(player, get_health) as f32;
        let p_local = Player::get_local().unwrap();
        let weapon = p_local.weapon();

        let mut hitbox_order_prio = if weapon.can_headshot() {
            HitboxPriority::PrioHead
        } else {
            HitboxPriority::All
        };
        if matches!(hitbox_order_prio, HitboxPriority::PrioHead) {
            if let Ok(gun) = weapon.as_gun() {
                if *setting!(aimbot, wait_for_charge) {
                    if hp > gun.get_damage(true) {
                        return Ok(());
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
            if let Some(target) = best_target {
                if target.prio.hitbox > hitbox_prio {
                    continue;
                }
            }

            let Some((point,point_prio)) = self.point_scan(&hitbox)? else {
                continue
            };

            if let Some(target) = best_target {
                if target.prio.point > point_prio && target.prio.hitbox == hitbox_prio {
                    continue;
                }
            }
            let target = Target {
                point,
                ent: id,
                hitbox_id: hitbox.id,
                prio: Priority {
                    ent: player_prioroty,
                    hitbox: hitbox_prio,
                    point: point_prio,
                },
            };
            *best_target = Some(target);
        }
        Ok(())
    }

    pub fn find_valid_player_target(&self, best_target: &mut Option<Target>) -> OxideResult<()> {
        for id in o!()
            .last_entity_cache
            .as_ref()
            .unwrap()
            .get_class_ids(ClassId::CTFPlayer)
        {
            self.scan_player_hitboxes(id, best_target)?;
        }
        Ok(())
    }
}
