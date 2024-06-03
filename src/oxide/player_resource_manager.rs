use std::mem::transmute;

use crate::{
    interface, o,
    sdk::{
        entity::Team, interfaces::entity::Entity, networkable::ClassId,
        player_resource::PlayerResource,
    },
    util::str_from_arr,
    vmt_call,
};

use super::entity_cache::EntityCache;

#[derive(Debug)]
pub struct PlayerResourceManager {
    pub entity: Option<&'static mut PlayerResource>,
}

impl PlayerResourceManager {
    pub fn new() -> Self {
        Self { entity: None }
    }
    pub fn update(&mut self, cache: &EntityCache) {
        let id = cache.get_class_ids(ClassId::CTFPlayerResource)[0];
        unsafe {
            self.entity = Some(transmute(Entity::get_ent(id)));
        }
    }
    pub fn all(&self) -> Vec<PlayerResourceData> {
        let mut players = Vec::new();
        if o!().player_resource_manager.entity.is_none() {
            return players;
        }
        for i in (0..vmt_call!(interface!(base_engine), get_max_clients)) {
            let Some(player) = PlayerResourceData::new(i as usize) else { continue; };
            players.push(player);
        }
        players
    }
}
#[derive(Debug, Clone)]
pub struct PlayerResourceData {
    pub damage: i32,
    pub name: String,
    pub team: Team,
    pub id: i32,
    pub connected: bool,
}
impl PlayerResourceData {
    pub fn new(id: usize) -> Option<PlayerResourceData> {
        let pr = o!().player_resource_manager.entity.as_mut().unwrap();

        if !pr.get_valid()[id] {
            return None;
        }

        let name_ptr = pr.get_name()[id];

        let mut name = if !name_ptr.is_null() {
            if let Ok(name) = unsafe { std::ffi::CStr::from_ptr(name_ptr).to_str() } {
                name
            } else {
                "null"
            }
        } else {
            "null"
        }
        .to_string();

        Some(PlayerResourceData {
            damage: pr.get_damage_resource()[id].clone(),
            name: str_from_arr(unsafe { name.as_mut_vec() }),
            team: pr.get_team()[id].clone().into(),
            id: pr.get_account_id()[id],
            connected: pr.get_connected()[id],
        })
    }
}
