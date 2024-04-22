use crate::{
    vmt_call,
    error::OxideResult,
    o,
    sdk::{entity::Entity, networkable::ClassId},
};

use super::{priority::Priority, Aimbot, Target};

impl Aimbot {
    pub fn find_sentry(&self) -> OxideResult<Option<Target>> {
        let mut best_target: Option<Target> = None;
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
            let Some(ent_prio) = self.ent_priority(&mut sentry)? else {
                continue;
            };
            if let Some(best_target) = &best_target {
                if best_target.prio.ent > ent_prio {
                    continue;
                }
            }

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

                if let Some(best_target) = &best_target {
                    if best_target.prio.point > point_prio{
                        continue;
                    }
                }
                let prio = Priority{ ent: ent_prio, hitbox: 0, point: point_prio };
                let target = Target{ point, ent: sentry, hitbox_id, prio };
                best_target = Some(target);
            }
        }
        Ok(best_target)
    }
}
