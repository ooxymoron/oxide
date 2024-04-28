use crate::{
    vmt_call,
    error::OxideResult,
    o,
    sdk::{
        entity::{pipe::PipeType, Entity},
        interfaces::model_info::HitboxId,
        networkable::ClassId,
    },
};

use super::{priority::Priority, Aimbot, Target};

impl Aimbot {
    pub fn find_sticky(&self) -> OxideResult<Option<Target>> {
        let mut best_target: Option<Target> = None;
        for id in o!()
            .last_entity_cache
            .clone()
            .unwrap()
            .get_ent(ClassId::CTFGrenadePipebombProjectile)
        {
            let Some(mut pipe) = Entity::get_ent(id) else {continue};
            if vmt_call!(pipe.as_networkable(), is_dormant) {
                continue;
            }
            if !matches!(
                *pipe.as_pipe()?.get_type(),
                PipeType::RemoteDetonate | PipeType::RemoteDetonatePractice
            ) {
                continue;
            }
            let Some(ent_prio) = self.sticky_priority(&mut pipe)? else {
                    continue;
                };
            if let Some(best_target) = &best_target {
                if best_target.prio.ent > ent_prio {
                    continue;
                }
            }
            let hitbox = pipe.get_hitboxes(vec![HitboxId::Head])?[0].clone();

            let Some((point,point_prio)) = self.point_scan(&hitbox)? else {
                continue;
            };

            if let Some(best_target) = &best_target {
                if best_target.prio.point > point_prio{
                    continue;
                }
            }
            let prio = Priority{ ent: ent_prio, hitbox: 0, point: point_prio };
            let target = Target{ point, ent: pipe, hitbox_id:hitbox.id, prio };
            best_target = Some(target);
        }
        Ok(best_target)
    }
    pub fn sticky_priority(&self, ent: &mut Entity) -> OxideResult<Option<isize>> {
        let p_local = &*Entity::get_local().unwrap();
        if vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number) {
            return Ok(None);
        }
        if matches!(
            ent.as_pipe()?.get_type(),
            PipeType::RemoteDetonate | PipeType::RemoteDetonatePractice
        ) {
            return Ok(Some(0));
        }
        return Ok(None);
    }
}
