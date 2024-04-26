use crate::{
    error::OxideResult,
    interface,
    oxide::entity_cache::EntityCache,
    sdk::{condition::ConditionFlags, entity::Entity, networkable::ClassId},
    setting,
    vmt_call,
};

use super::Paint;

impl Paint {
    pub fn esp(&mut self, cache: &EntityCache) -> OxideResult<()> {
        if !vmt_call!(interface!(base_engine), is_in_game) || !setting!(visual, esp) {
            return Ok(());
        }
        let p_local = Entity::get_local()?;
        let conditions = vec![
            ConditionFlags::Ubercharged,
            ConditionFlags::Bonked,
            ConditionFlags::Aiming,
            ConditionFlags::Disguised,
            ConditionFlags::Cloaked,
            ConditionFlags::Taunting,
            ConditionFlags::DeadRingered,
            ConditionFlags::Dazed,
            ConditionFlags::Charging,
            ConditionFlags::CritCola,
            ConditionFlags::Jarated,
            ConditionFlags::DefenseBuffed,
            ConditionFlags::Buffed,
            ConditionFlags::Milked,
            ConditionFlags::MarkedForDeath,
            ConditionFlags::RegenBuffed,
            ConditionFlags::UberBulletResist,
            ConditionFlags::UberBlastResist,
            ConditionFlags::UberFireResist,
        ];
        for id in cache.get_ent(ClassId::CTFPlayer) {
            let Some(ent) = Entity::get_ent(id) else {
                continue;
            };
            if vmt_call!(ent.as_networkable(), is_dormant) {
                continue;
            }
            if ent as *const _ == p_local.as_ent() as *const _ || !vmt_call!(ent, is_alive) {
                continue;
            }
            if !setting!(visual, esp_friendlies)
                && vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number)
            {
                continue;
            }

            let player = ent.as_player()?;
            let player_cond = player.get_condition();
            let conditions = conditions
                .iter()
                .filter_map(|&cond| {
                    if !player_cond.get(cond) {
                        return None;
                    }
                    return Some(format!("{:?}", cond));
                })
                .collect::<Vec<_>>();

            let info = player.info()?;
            let name = info.name;
            ent.paint(true, true, Some(&name), conditions);
        }
        if setting!(visual, esp_sentreis) {
            for id in cache.get_ent(ClassId::CObjectSentrygun) {
                let Some(ent) = Entity::get_ent(id) else{
                    continue;
                };
                if vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number)
                    && !setting!(visual, esp_friendlies)
                {
                    continue;
                }
                let obj = ent.as_object()?;
                let text = if *obj.get_mini() {
                    vec!["MINI".to_owned()]
                } else {
                    vec![format!("LEVEL: {:?}", obj.get_level())]
                };
                ent.paint(true, true, Some("sentry"), text);
            }
        }

        if setting!(visual, esp_projectiles) {
            for id in cache.get_ent(ClassId::CTFProjectileRocket) {
                let Some(ent) = Entity::get_ent(id) else{
                    continue;
                };
                if vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number)
                    && !setting!(visual, esp_friendlies)
                {
                    continue;
                }
                ent.paint(false, false, Some("rocket"), vec![]);
            }
            for id in cache.get_ent(ClassId::CTFGrenadePipebombProjectile) {
                let Some(ent) = Entity::get_ent(id) else{
                    continue;
                };
                if vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number)
                    && !setting!(visual, esp_friendlies)
                {
                    continue;
                }
                let text = ent.as_pipe()?.get_type().to_str();
                ent.paint(false, false, Some(text), vec![]);
            }
        }
        //for (name,ents) in cache.entities.clone().iter() {
        //    for id in ents {
        //        let Ok(entity) = Entity::get_ent(*id) else{
        //            continue;
        //        };
        //        entity.paint(&format!("{:?}",name));
        //    }

        //}

        Ok(())
    }
}
