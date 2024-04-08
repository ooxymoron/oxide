use crate::{
    vmt_call,
    error::OxideResult,
    o,
    sdk::{
        condition::ConditionFlags,
        entity::{player::Player, Entity},
        model_info::HitboxId,
        networkable::ClassId,
        weapon::{ItemDefiniitonIndex, WeaponType},
    },
    setting,
};

use super::{Aimbot, TargetData};

impl Aimbot {
    pub fn hitbox_order(&self, ent: &Entity) -> Vec<HitboxId> {
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
                        weapon.item_definition_index,
                        ItemDefiniitonIndex::SpyMTheAmbassador
                            | ItemDefiniitonIndex::SpyMFestiveAmbassador
                    ))
                    && vmt_call!(weapon.as_gun(), get_projectile_damage) >= vmt_call!(ent, get_health) as f32;
            }
            return true;
        })();
        if baim {
            vec![
                HitboxId::Pelvis,
                HitboxId::LeftFoot,
                HitboxId::RightFoot,
                HitboxId::Head,
            ]
        } else {
            vec![HitboxId::Head]
        }
    }
    pub fn player_prioroty(player: Player) -> OxideResult<Option<isize>> {
        let mut ignore_flags = vec![
            ConditionFlags::Ubercharged,
            ConditionFlags::UberchargeFading,
            ConditionFlags::Bonked,
        ];
        if !player.condition.get(ConditionFlags::CloakFlicker) {
            if !setting!(aimbot, target_invisible) {
                ignore_flags.push(ConditionFlags::Cloaked)
            }
            if !setting!(aimbot, target_disguised) {
                ignore_flags.push(ConditionFlags::Disguised)
            }
        }

        if ignore_flags
            .into_iter()
            .any(|flag| player.condition.get(flag))
        {
            return Ok(None);
        }
        Ok(Some(1))
    }

    pub fn find_player(&self) -> OxideResult<Option<TargetData>> {
        let mut target = None;
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

            let Some(prio) = self.ent_priority(player)? else {
                continue;
            };

            if let Some((_, _, _, last_prio, _)) = target {
                if last_prio > prio {
                    continue;
                }
            }
            let mut best_point = None;
            for hitbox_id in self.hitbox_order(player) {
                let hitbox = player.get_hitbox(hitbox_id).unwrap();

                let Some((point,point_prio)) = self.point_scan(player, hitbox_id, &hitbox)? else {
                    continue;
                };

                let Some((_, last_point_prio,_)) = best_point else {
                    best_point = Some((point, point_prio, hitbox_id));
                    continue;
                };
                if last_point_prio < point_prio {
                    best_point = Some((point, point_prio, hitbox_id));
                }
            }
            if let Some((point, point_prio, hitbox_id)) = best_point {
                let Some((_, _,_,last_prio, last_point_prio)) = &target else {
                target = Some((point, player, hitbox_id, prio, point_prio));
                    continue;
                };
                if prio > *last_prio || (prio == *last_prio && *last_point_prio < point_prio) {
                    target = Some((point, player, hitbox_id, prio, point_prio));
                }
            }
        }
        Ok(target)
    }
}
