use crate::{
    error::OxideResult,
    o,
    sdk::{
        condition::ConditionFlags,
        entity::{player::Player, weapon::ids::{ItemDefiniitonIndex, WeaponType}, Entity},
        model_info::HitboxId,
        networkable::ClassId,
    },
    setting, vmt_call,
};

use super::{priority::Priority, Aimbot, Target};

impl Aimbot {
    pub fn hitbox_order(&self, ent: &Entity) -> Vec<(isize, HitboxId)> {
        let p_local = &*Entity::get_local().unwrap();
        let weapon = vmt_call!(p_local.as_ent(), get_weapon);
        let id = vmt_call!(weapon, get_weapon_id);
        let baim = (|| {
            if weapon.can_headshot() {
                return setting!(aimbot, baim_if_lethal)
                    && (matches!(
                        id,
                        WeaponType::Sniperrifle
                            | WeaponType::SniperrifleClassic
                            | WeaponType::SniperrifleDecap
                    ) || matches!(
                        *weapon.get_item_definition_index(),
                        ItemDefiniitonIndex::SpyMTheAmbassador
                            | ItemDefiniitonIndex::SpyMFestiveAmbassador
                    ))
                    && vmt_call!(weapon.as_gun(), get_projectile_damage)
                        >= vmt_call!(ent, get_health) as f32;
            }
            return true;
        })();
        if baim {
            vec![
                (2, HitboxId::Pelvis),
                (0, HitboxId::Head),
                (0, HitboxId::LeftFoot),
                (0, HitboxId::RightFoot),
            ]
        } else {
            vec![
                (2, HitboxId::Head),
                (1, HitboxId::Pelvis),
                (0, HitboxId::LeftFoot),
                (0, HitboxId::RightFoot),
            ]
        }
    }
    pub fn player_prioroty(player: Player) -> OxideResult<Option<isize>> {
        let mut ignore_flags = vec![
            ConditionFlags::Ubercharged,
            ConditionFlags::UberchargeFading,
            ConditionFlags::Bonked,
        ];
        if !player.get_condition().get(ConditionFlags::CloakFlicker) {
            if !setting!(aimbot, target_invisible) {
                ignore_flags.push(ConditionFlags::Cloaked)
            }
            if !setting!(aimbot, target_disguised) {
                ignore_flags.push(ConditionFlags::Disguised)
            }
        }

        if ignore_flags
            .into_iter()
            .any(|flag| player.get_condition().get(flag))
        {
            return Ok(None);
        }
        Ok(Some(1))
    }

    pub fn find_player(&self) -> OxideResult<Option<Target>> {
        let mut best_target: Option<Target> = None;
        for id in o!()
            .last_entity_cache
            .clone()
            .unwrap()
            .get_ent(ClassId::CTFPlayer)
        {
            let Some(player) = Entity::get_ent(id) else {continue};
            if vmt_call!(player.as_networkable(), is_dormant) {
                continue;
            }

            let Some(ent_prio) = self.ent_priority(player)? else {
                continue;
            };
            if let Some(best_target) = &best_target {
                if best_target.prio.ent > ent_prio {
                    continue;
                }
            }

            for (hitbox_prio, hitbox_id) in self.hitbox_order(player) {
                if let Some(target) = &best_target {
                    if target.prio.hitbox > ent_prio {
                        break;
                    }
                }
                let hitbox = player.get_hitbox(hitbox_id).unwrap();

                let Some((point,point_prio)) = self.point_scan(player, hitbox_id, &hitbox)? else {
                    continue;
                };

                if let Some(target) = &best_target {
                    if target.prio.point > point_prio {
                        continue;
                    }
                }
                let prio = Priority {
                    ent: ent_prio,
                    hitbox: hitbox_prio,
                    point: point_prio,
                };
                let target = Target {
                    point,
                    ent: player,
                    hitbox_id,
                    prio,
                };
                best_target = Some(target);
            }
        }
        Ok(best_target)
    }
}
