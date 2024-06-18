use crate::{
    error::OxideResult,
    get_cheat, o,
    oxide::cheat::aimbot::Aimbot,
    sdk::{
        entity::{
            pipe::PipeType,
            player::{player_class::PlayerClass, Player},
        },
        interfaces::entity::Entity,
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
            if vmt_call!(ent.as_networkable(), is_dormant) {
                continue;
            }
            if !matches!(*ent.as_pipe()?.get_type(), PipeType::RemoteDetonate) {
                continue;
            }
            if ent.as_pipe()?.get_owner().resolve().unwrap() != p_local.as_ent() {
                continue;
            }
            stickies.push((*ent.get_origin(), ent.as_pipe()?.get_radius()))
        }
        if stickies.is_empty() {
            return Ok(false);
        }

        for id in o!()
            .last_entity_cache
            .as_ref()
            .unwrap()
            .get_class_ids(ClassId::CTFPlayer)
        {
            let Some(player) = Entity::get_ent(id) else {continue;};
            if vmt_call!(player.as_networkable(), is_dormant) || !vmt_call!(player, is_alive) {
                continue;
            }
            if get_cheat!(Aimbot)
                .player_prioroty(player.as_player()?)?
                .is_none()
            {
                continue;
            }
            for &(pos, radius) in &stickies {
                if (*player.get_origin() - pos).len() <= radius {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    pub fn create_move(&mut self, cmd: &mut UserCmd) -> OxideResult<()> {
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
