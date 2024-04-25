use crate::{
    error::OxideResult,
    o,
    sdk::{
        condition::ConditionFlags,
        entity::{player::Player, weapon::ids::{ItemDefiniitonIndex, WeaponType}, Entity},
        model_info::{Hitbox, HitboxId, HitboxWrapper},
        networkable::ClassId,
    },
    setting, vmt_call,
};

use super::{priority::Priority, Aimbot, Target};

impl Aimbot {
    pub fn hitbox_order(&self, ent: &Entity) -> Vec<(isize, HitboxWrapper)> {
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



        let hitboxes = ent.get_hitboxes(vec![HitboxId::Pelvis,HitboxId::Head,HitboxId::LeftFoot,HitboxId::RightFoot]).unwrap();
        if baim {
            vec![
                (2, hitboxes[0]),
                (0, hitboxes[1]),
                (0, hitboxes[2]),
                (0, hitboxes[3]),
            ]
        } else {
            vec![
                (2, hitboxes[1]),
                (1, hitboxes[0]),
                (0, hitboxes[2]),
                (0, hitboxes[3]),
            ]
        }
    }
    pub fn player_prioroty(&self,player: &Player) -> OxideResult<Option<isize>> {
        if self.ent_priority(player.as_ent())?.is_none() {
            return Ok(None)
        }
        let mut ignore_flags = vec![
            ConditionFlags::Ubercharged,
            ConditionFlags::UberchargeFading,
            ConditionFlags::Bonked,
        ];
        let spy_revealing_flags = vec![
            ConditionFlags::Jarated,
            ConditionFlags::Milked,
            ConditionFlags::CloakFlicker,
            ConditionFlags::OnFire,
            ConditionFlags::Bleeding,
        ];
        let conditions = player.get_condition();

        if spy_revealing_flags.into_iter()
            .all(|flag| !conditions.get(flag)) {
            if !setting!(aimbot, target_invisible) {
                ignore_flags.push(ConditionFlags::Cloaked)
            }
            if !setting!(aimbot, target_disguised) {
                ignore_flags.push(ConditionFlags::Disguised)
            }
        }

        if ignore_flags
            .into_iter()
            .any(|flag| conditions.get(flag))
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
            if vmt_call!(player.as_networkable(), is_dormant) || !vmt_call!(player, is_alive){
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

            for (hitbox_prio, hitbox) in self.hitbox_order(player) {
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
