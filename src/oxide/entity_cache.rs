use std::collections::HashMap;

use crate::{
    error::{OxideError, OxideResult},
    interface, o,
    sdk::{entity::Entity, interfaces::model_info::HitboxWrapper, networkable::ClassId},
    vmt_call,
};

#[derive(Debug)]
pub struct EntityCache {
    pub entities: HashMap<ClassId, Vec<u32>>,
    hitboxes: HashMap<u32, Vec<HitboxWrapper>>,
}

impl EntityCache {
    pub fn init() -> OxideResult<EntityCache> {
        let entity_count = vmt_call!(interface!(entity_list), get_max_entities);

        let mut entities: HashMap<ClassId, Vec<u32>> = HashMap::new();

        for id in 0..entity_count {
            let Some(ent) = Entity::get_ent(id) else {
                continue;
            };
            let net = ent.as_networkable();
            let class = net.get_client_class();
            if let Some(vec) = entities.get_mut(&class.class_id) {
                vec.push(id);
            } else {
                entities.insert(class.class_id.clone(), vec![id]);
            };
        }

        Ok(EntityCache {
            entities,
            hitboxes: HashMap::new(),
        })
    }
    pub fn get_hitboxes(&mut self, id: u32) -> OxideResult<&mut Vec<HitboxWrapper>> {
        if self.hitboxes.contains_key(&id) {
            return Ok(self.hitboxes.get_mut(&id).unwrap());
        }

        let Some(ent) = Entity::get_ent(id) else {
                return Err(OxideError::new("null ent"));
            };
        self.hitboxes.insert(id, ent.calculate_hitboxes().unwrap());
        return Ok(self.hitboxes.get_mut(&id).unwrap());
    }
    pub fn get_ent(&self, id: ClassId) -> Vec<u32> {
        self.entities.get(&id).cloned().unwrap_or(vec![])
    }
}
