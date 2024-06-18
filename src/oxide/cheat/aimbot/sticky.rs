use crate::{
    error::OxideResult,
    o,
    sdk::{
        entity::{pipe::PipeType, player::Player, Entity},
        networkable::ClassId,
    },
    setting, vmt_call,
};

use super::{priority::Priority, Aimbot, Target};

impl Aimbot {
    pub fn scan_sticky_hitboxes(
        &self,
        id: i32,
        best_target: &mut Option<Target>,
    ) -> OxideResult<()> {
        let Some(mut pipe) = Entity::get_ent(id) else {return Ok(());};
        let Some(ent_prio) = self.sticky_priority(&mut pipe)? else {
            return Ok(());
        };
        if let Some(best_target) = &best_target {
            if best_target.prio.ent > ent_prio {
                return Ok(());
            }
        }
        let hitbox = pipe.get_hitbox(0)?;

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
    pub fn find_sticky(&self, best_target: &mut Option<Target>) -> OxideResult<()> {
        if !*setting!(aimbot, target_stickies) {
            return Ok(());
        }
        for id in o!()
            .last_entity_cache
            .as_ref()
            .unwrap()
            .get_class_ids(ClassId::CTFGrenadePipebombProjectile)
        {
            self.scan_sticky_hitboxes(id, best_target)?;
        }
        Ok(())
    }
    pub fn sticky_priority(&self, ent: &mut Entity) -> OxideResult<Option<isize>> {
        if vmt_call!(ent.as_networkable(), is_dormant) {
            return Ok(None);
        }
        if !matches!(*ent.as_pipe()?.get_type(), PipeType::RemoteDetonate) {
            return Ok(None);
        }
        let p_local = Player::get_local().unwrap();
        if vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number) {
            return Ok(None);
        }
        if !*ent.as_pipe().unwrap().get_touched() {
            return Ok(None);
        }
        return Ok(Some(0));
    }
}
