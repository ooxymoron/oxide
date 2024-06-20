use std::ptr::addr_of;

use crate::{
    error::OxideResult,
    o,
    sdk::{
        entity::{
            pipe::PipeType,
            player::{player_class::PlayerClass, Player},
        },
        interfaces::{
            engine_trace::{trace, TraceFilter, MASK_SHOT, MASK_SHOT_HULL},
            entity::Entity,
        },
        networkable::ClassId,
        user_cmd::{ButtonFlags, UserCmd},
    },
    setting, vmt_call,
};

use super::Cheat;

#[derive(Debug, Clone)]
pub struct Triggerbot {}

impl Triggerbot {
    pub fn init() -> Triggerbot {
        Triggerbot {}
    }
    pub fn should_detonate_stickies(&mut self) -> OxideResult<bool> {
        let p_local = Player::get_local().unwrap();

        let mut stickies = Vec::new();

        for id in o!()
            .last_entity_cache
            .as_ref()
            .unwrap()
            .get_class_ids(ClassId::CTFGrenadePipebombProjectile)
        {
            let Some(ent) = Entity::get_ent(id) else {return Ok(false);};
            if !matches!(*ent.as_pipe()?.get_type(), PipeType::RemoteDetonate) {
                continue;
            }
            if ent.as_pipe()?.get_owner().resolve().unwrap() != p_local.as_ent() {
                continue;
            }
            if ent.as_pipe()?.get_radius().is_none() {
                continue;
            }

            stickies.push(ent)
        }
        if stickies.is_empty() {
            return Ok(false);
        }
        if *setting!(triggerbot, avoid_self_damage) {
            let p_local = Player::get_local().unwrap();
            for sticky in &mut stickies {
                let player_pos = vmt_call!(p_local.as_ent(), world_space_center);
                if (*player_pos - *sticky.get_origin()).len()
                    <= sticky.as_pipe()?.get_radius().unwrap()
                {
                    let filter = TraceFilter::new(p_local.as_ent());
                    let trace = trace(player_pos, sticky.get_origin(), MASK_SHOT_HULL, &filter);
                    if trace.entity == addr_of!(**sticky) {
                        return Ok(false);
                    }
                }
            }
        }

        for id in o!()
            .last_entity_cache
            .as_ref()
            .unwrap()
            .get_class_ids(ClassId::CTFPlayer)
        {
            let Some(ent) = Entity::get_ent(id) else {continue;};
            if ent.priority().is_none() {
                continue;
            }
            if self.sticky_range_check(ent, &mut stickies)? {
                return Ok(true);
            }
        }
        if *setting!(aimbot, target_sentries) {
            for id in o!()
                .last_entity_cache
                .as_ref()
                .unwrap()
                .get_class_ids(ClassId::CObjectSentrygun)
            {
                let Some(ent) = Entity::get_ent(id) else {continue;};
                if ent.priority().is_none() {
                    continue;
                }
                if self.sticky_range_check(ent, &mut stickies)? {
                    return Ok(true);
                }
            }
        }
        if *setting!(aimbot, target_buildings) {
            for id in o!()
                .last_entity_cache
                .as_ref()
                .unwrap()
                .get_class_ids(ClassId::CObjectTeleporter)
            {
                let Some(ent) = Entity::get_ent(id) else {continue;};
                if ent.priority().is_none() {
                    continue;
                }
                if self.sticky_range_check(ent, &mut stickies)? {
                    return Ok(true);
                }
            }
            for id in o!()
                .last_entity_cache
                .as_ref()
                .unwrap()
                .get_class_ids(ClassId::CObjectDispenser)
            {
                let Some(ent) = Entity::get_ent(id) else {continue;};
                if ent.priority().is_none() {
                    continue;
                }
                if self.sticky_range_check(ent, &mut stickies)? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
    pub fn sticky_range_check(
        &self,
        ent: &Entity,
        stickies: &mut Vec<&mut Entity>,
    ) -> OxideResult<bool> {
        if vmt_call!(ent.as_networkable(), is_dormant) || !vmt_call!(ent, is_alive) {
            return Ok(false);
        }
        for sticky in stickies {
            let player_pos = vmt_call!(ent, world_space_center);
            if (*player_pos - *sticky.get_origin()).len() <= sticky.as_pipe()?.get_radius().unwrap()
            {
                let filter = TraceFilter::new(sticky);
                let trace = trace(sticky.get_origin(), player_pos, MASK_SHOT, &filter);
                if trace.entity == addr_of!(*ent) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    pub fn create_move(&mut self, cmd: &mut UserCmd) -> OxideResult<()> {
        if o!().last_entity_cache.is_none() {
            return Ok(());
        }
        if cmd.buttons.get(ButtonFlags::InAttack2) {
            return Ok(());
        }
        let p_local = Player::get_local().unwrap();

        if *setting!(triggerbot, sticky)
            && *p_local.get_player_class() == PlayerClass::Demoman
            && self.should_detonate_stickies()?
        {
            cmd.buttons.set(ButtonFlags::InAttack2, true);
            return Ok(());
        }
        Ok(())
    }
}

impl Cheat for Triggerbot {}
