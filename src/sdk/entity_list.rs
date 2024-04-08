use crate::cfn;

use super::{entity::Entity, CBaseHandle, WithVmt};


pub type EntityList = WithVmt<VMTEntityList>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTEntityList {
    _pad1: [u32; 3],
    pub get_client_entity: cfn!(*mut Entity, *const EntityList, isize),
	pub get_client_entity_from_handle: cfn!(&'static Entity, *const EntityList, CBaseHandle ),
	pub number_of_entities: cfn!(isize, *const EntityList, bool ),
	pub get_highest_entity_index: cfn!(isize, *const EntityList),
	pub set_max_entities: cfn!((), *const EntityList),
    pub get_max_entities: cfn!(isize, *const EntityList),

}

