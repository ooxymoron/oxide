use std::ptr::null;

use sdl2_sys::SDL_Scancode;

use crate::{
    draw::event::EventType,
    error::OxideResult,
    o, s,
    sdk::{
        condition::ConditionFlags,
        entity::{Entity, ObserverMode},
        networkable::ClassId,
        view_setup::ViewSetup,
    },
    setting,
    util::arcm::Arcm,
    vmt_call,
};

use super::Cheat;

#[derive(Debug)]
pub struct Visuals {
    pub spectators: Arcm<Vec<(String, ObserverMode)>>,
    pub tp_offset_key_held: bool,
}

impl Visuals {
    pub fn init() -> Visuals {
        Visuals {
            spectators: Arcm::new(vec![]),
            tp_offset_key_held: false,
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
        let p_local = Entity::get_local()?;
        let ent = if vmt_call!(p_local.as_ent(), is_alive) {
            p_local.as_ent()
        } else {
            vmt_call!(p_local.as_ent(), get_observer_target)
        };
        #[allow(useless_ptr_null_checks)]
        if ent as *const _ == null() {
            return Ok(());
        }
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
            if p_local as *const _ as *const () == spectator as *const _ as *const ()
                || target != ent
            {
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
                    .get_condition()
                    .get(ConditionFlags::Disguised)
            {
                continue;
            }
            player
                .as_player()
                .unwrap()
                .get_condition()
                .set(ConditionFlags::Disguised, false)
        }
        Ok(())
    }
    pub fn override_view(&mut self, view_setup: &mut ViewSetup) {
        let Ok(p_local) = Entity::get_local() else { return };
        if !vmt_call!(p_local.as_ent(), is_alive) {
            return;
        }
        if !p_local.get_condition().get(ConditionFlags::Zoomed)
            || (p_local.get_condition().get(ConditionFlags::Zoomed)
                && setting!(visual, remove_zoom))
        {
            view_setup.fov = setting!(visual, fov);
        };
        let force_taunt_cam = p_local.get_force_taunt_cam();
        if setting!(visual, third_person) {
            let dirs = vmt_call!(p_local.as_ent(), get_abs_angles).to_vectors();
            *force_taunt_cam = true;
            let x = setting!(visual, tp_offset_x);
            let y = setting!(visual, tp_offset_y);
            let z = setting!(visual, tp_offset_z);
            view_setup.origin.x += x * dirs[0].x;
            view_setup.origin.y += x * dirs[1].x;
            view_setup.origin.z += x * dirs[2].x;
            view_setup.origin.x += y * dirs[0].y;
            view_setup.origin.y += y * dirs[1].y;
            view_setup.origin.z += y * dirs[2].y;
            view_setup.origin.x += z * dirs[0].z;
            view_setup.origin.y += z * dirs[1].z;
            view_setup.origin.z += z * dirs[2].z;
        } else {
            *force_taunt_cam = false;
        }
    }
}
impl Cheat for Visuals {
    fn handle_event(&mut self, event: &mut crate::draw::event::Event) {
        let tp_key = setting!(visual, tp_key);
        let tp_offset_key = setting!(visual, tp_offset_key);
        match event.r#type {
            EventType::KeyDown(key) => {
                if key == *tp_key {
                    let mut tp = s!().visual.third_person.lock().unwrap();
                    *tp = !*tp;
                    event.handled = true;
                }
                if key == *tp_offset_key {
                    self.tp_offset_key_held = true;
                    event.handled = true;
                }
                if self.tp_offset_key_held {
                    {
                        let mut x = s!().visual.tp_offset_x.lock().unwrap();
                        let mut y = s!().visual.tp_offset_y.lock().unwrap();
                        let mut z = s!().visual.tp_offset_z.lock().unwrap();
                        match key {
                            SDL_Scancode::SDL_SCANCODE_UP    => *x += 10.0,
                            SDL_Scancode::SDL_SCANCODE_DOWN  => *x -= 10.0,
                            SDL_Scancode::SDL_SCANCODE_LEFT  => *y += 10.0,
                            SDL_Scancode::SDL_SCANCODE_RIGHT => *y -= 10.0,
                            SDL_Scancode::SDL_SCANCODE_SPACE => *z += 10.0,
                            SDL_Scancode::SDL_SCANCODE_LCTRL => *z -= 10.0,
                            _ => {}
                        }
                    }
                }
            }
            EventType::KeyUp(key) => {
                if key == *tp_offset_key {
                    self.tp_offset_key_held = false;
                    event.handled = true;
                }
            }
            _ => (),
        }
    }
}
