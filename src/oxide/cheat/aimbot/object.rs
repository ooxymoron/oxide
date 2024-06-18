use crate::{
    error::OxideResult,
    o,
    sdk::{entity::Entity, networkable::ClassId},
    setting, vmt_call,
};

use super::{priority::Priority, Aimbot, Target};

impl Aimbot {
    pub fn scan_sentry_hitboxes(
        &self,
        id: i32,
        best_target: &mut Option<Target>,
    ) -> OxideResult<()> {
        let Some(sentry) = Entity::get_ent(id) else {return Ok(());};
        if vmt_call!(sentry.as_networkable(), is_dormant) {
            return Ok(());
        }
        let Some(ent_prio) = sentry.priority() else {
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
    pub fn scan_object_hitboxes(
        &self,
        id: i32,
        best_target: &mut Option<Target>,
    ) -> OxideResult<()> {
        let Some(object) = Entity::get_ent(id) else {return Ok(());};
        if vmt_call!(object.as_networkable(), is_dormant) {
            return Ok(());
        }
        let Some(ent_prio) = object.priority() else {
                return Ok(());
            };
        if let Some(best_target) = &best_target {
            if best_target.prio.ent > ent_prio {
                return Ok(());
            }
        }

        let hitbox = object.get_hitbox(0)?;

        let Some((point,point_prio)) = self.point_scan(&hitbox)? else {
                return Ok(());
            };

        if let Some(best_target) = &best_target {
            if best_target.prio.point > point_prio {
                return Ok(());
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
        Ok(())
    }
    pub fn find_object(&self, best_target: &mut Option<Target>) -> OxideResult<()> {
        if *setting!(aimbot, target_sentries) {
            for id in o!()
                .last_entity_cache
                .as_ref()
                .unwrap()
                .get_class_ids(ClassId::CObjectSentrygun)
            {
                self.scan_sentry_hitboxes(id, best_target)?;
            }
        }
        if *setting!(aimbot, target_buildings) {
            for id in o!()
                .last_entity_cache
                .as_ref()
                .unwrap()
                .get_class_ids(ClassId::CObjectDispenser)
            {
                self.scan_object_hitboxes(id, best_target)?;
            }
            for id in o!()
                .last_entity_cache
                .as_ref()
                .unwrap()
                .get_class_ids(ClassId::CObjectTeleporter)
            {
                self.scan_object_hitboxes(id, best_target)?;
            }
        }
        Ok(())
    }
}
