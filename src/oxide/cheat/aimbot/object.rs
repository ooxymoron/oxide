use crate::{
    error::OxideResult, o, sdk::{entity::Entity, networkable::ClassId}, setting, vmt_call
};

use super::{priority::Priority, Aimbot, Target};

impl Aimbot {
    pub fn scan_object_hitboxes(
        &self,
        id: i32,
        best_target: &mut Option<Target>,
    ) -> OxideResult<()> {
        let Some(mut sentry) = Entity::get_ent(id) else {return Ok(());};
        if vmt_call!(sentry.as_networkable(), is_dormant) {
            return Ok(());
        }
        let Some(ent_prio) = self.ent_priority(&mut sentry)? else {
                return Ok(());
            };
        if let Some(best_target) = &best_target {
            if best_target.prio.ent > ent_prio {
                return Ok(());
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
                ent: id,
                hitbox_id: hitbox.id,
                prio,
            };
            *best_target = Some(target);
        }
        Ok(())
    }
    pub fn find_object(&self,best_target: &mut Option<Target>) -> OxideResult<()> {
        if !*setting!(aimbot, target_sentries) {
            return Ok(());
        }
        for id in o!()
            .last_entity_cache
            .as_ref()
            .unwrap()
            .get_class_ids(ClassId::CObjectSentrygun)
        {
            self.scan_object_hitboxes(id, best_target)?;
        }
        Ok(())
    }
}
