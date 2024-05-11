use crate::{
    error::OxideResult,
    o,
    sdk::{entity::Entity, networkable::ClassId},
    vmt_call,
};

use super::{priority::Priority, Aimbot, Target};

impl Aimbot {
    pub fn find_sentry(&self) -> OxideResult<Option<Target>> {
        let mut best_target: Option<Target> = None;
        for id in o!()
            .last_entity_cache
            .as_ref()
            .unwrap()
            .get_class_ids(ClassId::CObjectSentrygun)
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

            for hitbox in sentry.get_hitboxes()?.values_mut() {
                let Some((point,point_prio)) = self.point_scan(&hitbox)? else {
                    continue;
                };

                if let Some(best_target) = &best_target {
                    if best_target.prio.point > point_prio {
                        continue;
                    }
                }
                let prio = Priority {
                    ent: ent_prio,
                    hitbox: 0,
                    point: point_prio,
                };
                let target = Target {
                    point,
                    ent: sentry,
                    hitbox_id: hitbox.id,
                    prio,
                };
                best_target = Some(target);
            }
        }
        Ok(best_target)
    }
}
