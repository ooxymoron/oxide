use std::{collections::HashMap, mem::MaybeUninit};

use crate::{
    vmt_call,
    error::{OxideError, OxideResult},
    interface, o,
    sdk::{
        entity::{BoneMask, Bones, Entity, MAX_STUDIO_BONES},
        model_render::Matrix3x4,
        networkable::ClassId,
    },
};

#[derive(Debug, Clone)]
pub struct EntityCache {
    pub entities: HashMap<ClassId, Vec<u32>>,
    bones: HashMap<u32, [Matrix3x4; MAX_STUDIO_BONES]>,
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

        Ok(EntityCache { entities, bones: HashMap::new() })
    }
    pub fn get_bones(&mut self, id: u32) -> OxideResult<Bones> {
        if let Some(bones) = self.bones.get(&id) {
            return Ok(bones.clone())
        }
        
        let Some(ent) = Entity::get_ent(id) else {
            return Err(OxideError::new("none ent"));
        };
        let renderable = ent.as_renderable();

        let bones = unsafe { MaybeUninit::zeroed().assume_init() };
        vmt_call!(
            renderable,
            setup_bones,
            &bones,
            MAX_STUDIO_BONES,
            BoneMask::Hitbox,
            o!().global_vars.curtime
        );
        self.bones.insert(id, bones.clone());
        Ok(bones.clone())
    }
    pub fn get_ent(&self, id: ClassId) -> Vec<u32> {
        self.entities.get(&id).cloned().unwrap_or(vec![])
    }
}
