use crate::{
    error::OxideResult,
    hex_to_rgb, interface, o,
    oxide::entity_cache::EntityCache,
    rgb_to_hex,
    sdk::{
        entity::{player::Player, Entity},
        interfaces::model_info::{HitboxId, HitboxWrapper},
        networkable::ClassId,
    },
    setting,
    util::world_to_screen,
    vmt_call,
};

use super::Paint;

const COLOR_SCALE: f32 = 1.0 / 2.0;

impl Paint {
    pub fn draw_hitboxes(&mut self, cache: &EntityCache) -> OxideResult<()> {
        if !vmt_call!(interface!(base_engine), is_in_game) || !setting!(visual, hitboxes) {
            return Ok(());
        }
        let p_local = Player::get_local()?;
        for id in cache.get_ent(ClassId::CTFPlayer) {
            let Some(player) = Entity::get_ent(id ) else {continue};
            if vmt_call!(player.as_networkable(), is_dormant) || !vmt_call!(player, is_alive) {
                continue;
            }
            let team = vmt_call!(player, get_team_number);
            let (r, g, b) = hex_to_rgb!(team.color());
            let color = rgb_to_hex!(
                r as f32 * COLOR_SCALE,
                g as f32 * COLOR_SCALE,
                b as f32 * COLOR_SCALE
            );
            let hitboxes = player.get_hitboxes()?;
            for hitbox in hitboxes {
                self.draw_hitbox(hitbox, color, 30)?;
            }
        }
        for id in cache.get_ent(ClassId::CObjectSentrygun) {
            let Some(sentry) = Entity::get_ent(id ) else {continue};
            if vmt_call!(sentry.as_networkable(), is_dormant) {
                continue;
            }
            if sentry as *const _ == p_local.as_ent() as *const _ || !vmt_call!(sentry, is_alive) {
                continue;
            }
            let team = vmt_call!(sentry, get_team_number);

            let (r, g, b) = hex_to_rgb!(team.color());
            let color = rgb_to_hex!(
                r as f32 * COLOR_SCALE,
                g as f32 * COLOR_SCALE,
                b as f32 * COLOR_SCALE
            );
            let hitboxes = sentry.get_hitboxes()?;
            for hitbox in hitboxes {
                self.draw_hitbox(hitbox, color, 50)?;
            }
        }
        for id in cache.get_ent(ClassId::CTFGrenadePipebombProjectile) {
            let Some(pipe) = Entity::get_ent(id) else {continue};
            if vmt_call!(pipe.as_networkable(), is_dormant) {
                continue;
            }
            if pipe as *const _ == p_local.as_ent() as *const _ || !vmt_call!(pipe, is_alive) {
                continue;
            }
            let team = vmt_call!(pipe, get_team_number);

            let hitbox = pipe.get_hitbox(HitboxId::Head)?;
            self.draw_hitbox(hitbox, team.color(), 10)?;
        }
        Ok(())
    }
    pub fn draw_hitbox(
        &mut self,
        hitbox: &HitboxWrapper,
        color: usize,
        alpha: u8,
    ) -> OxideResult<()> {
        let corners = hitbox.corners()?;
        let corners = corners
            .iter()
            .map(|x| world_to_screen(x))
            .collect::<Vec<_>>();

        let pairs = [
            (&corners[0], &corners[1]),
            (&corners[0], &corners[2]),
            (&corners[0], &corners[4]),
            (&corners[7], &corners[3]),
            (&corners[7], &corners[5]),
            (&corners[7], &corners[6]),
            (&corners[2], &corners[3]),
            (&corners[2], &corners[6]),
            (&corners[1], &corners[5]),
            (&corners[1], &corners[3]),
            (&corners[4], &corners[6]),
            (&corners[4], &corners[5]),
        ];

        for pair in pairs {
            let Some(pos1) = &pair.0 else {
                continue;
            };
            let Some(pos2) = &pair.1 else {
                continue;
            };
            let (r, g, b) = hex_to_rgb!(color);
            vmt_call!(interface!(surface), set_color, r, g, b, alpha);

            vmt_call!(
                interface!(surface),
                draw_line,
                pos1.x as i32,
                pos1.y as i32,
                pos2.x as i32,
                pos2.y as i32
            );
        }
        Ok(())
    }
}
