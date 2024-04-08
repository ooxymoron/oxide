use std::ptr::null;

use crate::{
    vmt_call,
    draw::event::EventType,
    error::OxideResult,
    o, s,
    sdk::{
        condition::ConditionFlags,
        entity::{Entity, ObserverMode},
        networkable::ClassId,
    },
    setting,
    util::arcm::Arcm,
};

use super::Cheat;

#[derive(Debug)]
pub struct Visuals {
    pub spectators: Arcm<Vec<(String, ObserverMode)>>,
}

impl Visuals {
    pub fn init() -> Visuals {
        Visuals {
            spectators: Arcm::new(vec![]),
        }
    }
    pub fn name() -> &'static str {
        "Visuals"
    }
    pub fn net_update_end(&mut self) -> OxideResult<()> {
        self.remove_disguises()?;
        self.update_spectators()?;
        Ok(())
    }
    pub fn update_spectators(&mut self) -> OxideResult<()> {
        let p_local = &*Entity::get_local().unwrap();
        let ent = if vmt_call!(p_local.as_ent(), is_alive) {
            p_local.as_ent()
        } else {
            vmt_call!(p_local.as_ent(), get_observer_target)
        };
        let Some(cache)= &o!().last_entity_cache else {
            return Ok(())
        };
        let mut spectators = vec![];
        for id in cache.get_ent(ClassId::CTFPlayer) {
            let Some(spectator) = Entity::get_ent(id) else {continue};
            if vmt_call!(spectator.as_networkable(), is_dormant) {
                continue;
            }
            let mode = vmt_call!(spectator, get_observer_mode);
            if mode == ObserverMode::None {
                continue;
            }
            let target = vmt_call!(spectator, get_observer_target);
            #[allow(useless_ptr_null_checks)]
            if target as *const _ == null() {
                continue;
            }
            if p_local as *const _ as *const () == spectator as *const _ as *const () || target != ent {
                continue;
            }
            let info = spectator.as_player()?.info()?;
            spectators.push((info.name, mode));
        }
        let mut spectators_orig = self.spectators.lock().unwrap();
        *spectators_orig = spectators;
        Ok(())
    }
    pub fn remove_disguises(&self) -> OxideResult<()> {
        if !setting!(visual, remove_disguises) {
            return Ok(());
        }
        let p_local = &*Entity::get_local().unwrap();

        let local_team = vmt_call!(p_local.as_ent(), get_team_number);
        for id in o!()
            .last_entity_cache
            .clone()
            .unwrap()
            .get_ent(ClassId::CTFPlayer)
        {
            let Some(player) = Entity::get_ent(id) else {continue};
            if vmt_call!(player.as_networkable(), is_dormant) {
                continue;
            }
            if vmt_call!(player, get_team_number) == local_team
                || !player
                    .as_player()
                    .unwrap()
                    .condition
                    .get(ConditionFlags::Disguised)
            {
                continue;
            }
            player
                .as_player()
                .unwrap()
                .condition
                .set(ConditionFlags::Disguised, false)
        }
        Ok(())
    }
}
impl Cheat for Visuals {
    fn handle_event(&mut self, event: &mut crate::draw::event::Event) {
        let tp_key = setting!(visual, third_person_key);
        match event.r#type {
            EventType::KeyDown(key) => {
                if key == *tp_key {
                    let mut tp = s!().visual.third_person.lock().unwrap();
                    *tp = !*tp;
                }
            }
            _ => (),
        }
    }
}
