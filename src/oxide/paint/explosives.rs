use crate::{
    draw::colors::{RED, YELLOW},
    error::OxideResult,
    interface, o,
    sdk::{
        entity::{pipe::PipeType, player::Player},
        interfaces::entity::Entity,
        networkable::ClassId,
    },
    setting, vmt_call,
};

use super::{frame::PaintFrame, Paint};

impl Paint {
    pub fn explosives(&self, frame: &PaintFrame) -> OxideResult<()> {
        if !vmt_call!(interface!(base_engine), is_in_game) || !*setting!(visual, explosives) {
            return Ok(());
        }
        let Some(cache) = o!().last_entity_cache.as_ref() else {return Ok(())};
        let p_local = Player::get_local()?;

        for id in cache.get_class_ids(ClassId::CTFGrenadePipebombProjectile) {
            let Some(ent) = Entity::get_ent(id) else {
                continue;
            };
            if !matches!(*ent.as_pipe()?.get_type(), PipeType::RemoteDetonate) {
                continue;
            }
            if ent.as_pipe()?.get_owner().resolve().unwrap() != p_local.as_ent()
                && ent.priority().is_none()
            {
                continue;
            }
            let Some(radius) = ent.as_pipe()?.get_radius() else {continue;};
            let pos = *ent.get_origin();
            let color = if (*p_local.as_ent().get_origin() - pos).len() < radius {
                RED
            } else {
                YELLOW
            };

            frame.icosphere(pos, radius, color, 5);
        }
        Ok(())
    }
}
