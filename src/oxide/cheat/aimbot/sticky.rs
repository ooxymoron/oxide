use crate::{
    vmt_call,
    error::OxideResult,
    o,
    sdk::{
        entity::{pipe::PipeType, Entity},
        model_info::HitboxId,
        networkable::ClassId,
    },
};

use super::{Aimbot, TargetData};

impl Aimbot {
    pub fn find_sticy(&self) -> OxideResult<Option<TargetData>> {
        let mut target: Option<TargetData> = None;
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
                pipe.as_pipe()?.r#type,
                PipeType::RemoteDetonate | PipeType::RemoteDetonatePractice
            ) {
                continue;
            }
            let Some(prio) = self.sticky_priority(&mut pipe)? else {
                    continue;
                };
            let hitbox_id = HitboxId::Head;
            let hitbox = pipe.get_hitbox(hitbox_id).unwrap();

            let Some((point,point_prio)) = self.point_scan(pipe, HitboxId::Head, &hitbox)? else {
                continue;
            };

            let Some((_,_,_, last_prio, last_point_prio)) = &target else {
                target = Some((point, pipe, hitbox_id, prio, point_prio));
                continue;
            };
            if prio > *last_prio || (prio == *last_prio && *last_point_prio < point_prio) {
                target = Some((point, pipe, hitbox_id, prio, point_prio));
            }
        }
        Ok(target)
    }
    pub fn sticky_priority(&self, ent: &mut Entity) -> OxideResult<Option<isize>> {
        let p_local = &*Entity::get_local().unwrap();
        if vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number) {
            return Ok(None);
        }
        if matches!(
            ent.as_pipe()?.r#type,
            PipeType::RemoteDetonate | PipeType::RemoteDetonatePractice
        ) {
            return Ok(Some(0));
        }
        return Ok(None);
    }
}
