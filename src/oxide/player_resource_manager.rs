use std::mem::transmute;

use crate::sdk::{
    interfaces::entity::Entity, networkable::ClassId,
    player_resource::PlayerResource,
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
}
#[derive(Debug)]
pub struct PlayerResourceData {
    pub damage: i32,
}
