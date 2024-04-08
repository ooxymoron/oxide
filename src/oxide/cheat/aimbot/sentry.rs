use crate::{
    vmt_call,
    error::OxideResult,
    o,
    sdk::{entity::Entity, networkable::ClassId},
};

use super::{Aimbot, TargetData};

impl Aimbot {
    pub fn find_sentry(&self) -> OxideResult<Option<TargetData>> {
        let mut target = None;
        for id in o!()
            .last_entity_cache
            .clone()
            .unwrap()
            .get_ent(ClassId::CObjectSentrygun)
        {
            let Some(mut sentry) = Entity::get_ent(id) else {continue};
            if vmt_call!(sentry.as_networkable(), is_dormant) {
                continue;
            }
            let Some(prio) = self.ent_priority(&mut sentry)? else {
                continue;
            };
            if let Some((_, _, _, last_prio, _)) = target {
                if last_prio > prio {
                    continue;
                }
            }

            let mut best_point = None;
            for hitbox_id in sentry
                .as_object()
                .unwrap()
                .as_sentry()
                .unwrap()
                .get_hitbox_ids()
            {
                let hitbox = sentry.get_hitbox(hitbox_id).unwrap();

                let Some((point,point_prio)) = self.point_scan(sentry, hitbox_id, &hitbox)? else {
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
                target = Some((point, sentry, hitbox_id, prio, point_prio));
                    continue;
                };
                if prio > *last_prio || (prio == *last_prio && *last_point_prio < point_prio) {
                    target = Some((point, sentry, hitbox_id, prio, point_prio));
                }
            }
        }
        Ok(target)
    }
}
