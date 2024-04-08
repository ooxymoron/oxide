use crate::{
    vmt_call,
    error::OxideResult,
    hex_to_rgb, interface, o,
    oxide::entity_cache::EntityCache,
    rgb_to_hex, setting,
    sdk::{
        entity::Entity,
        model_info::{Hitbox, HitboxId},
        networkable::ClassId,
    },
    util::world_to_screen,
};

use super::Paint;

const COLOR_SCALE: f32 = 1.0 / 2.0;

impl Paint {
    pub fn draw_hitboxes(&mut self, cache: &EntityCache) -> OxideResult<()> {
        if !vmt_call!(interface!(base_engine), is_in_game) || !setting!(visual,hitboxes) {
            return Ok(());
        }
        //let hitbox_scale = setting!(aimbot,hitbox_scale);
        let p_local = Entity::get_local()?;
        for id in cache.get_ent(ClassId::CTFPlayer) {
            let Some(player) = Entity::get_ent(id) else {continue};
            if vmt_call!(player.as_networkable(), is_dormant) {
                continue;
            }
            if player as *const _ == p_local.as_ent() as *const _ || !vmt_call!(player, is_alive) {
                continue;
            }
            let team = vmt_call!(player, get_team_number);
            for hitbox_id in HitboxId::all() {
                let (r, g, b) = hex_to_rgb!(team.color());
                let color = rgb_to_hex!(
                    r as f32 * COLOR_SCALE,
                    g as f32 * COLOR_SCALE,
                    b as f32 * COLOR_SCALE
                );
                let hitbox = player.get_hitbox(hitbox_id).unwrap();
                self.draw_hitbox(&player, hitbox, color, 30)?;
            }
        }
        for id in cache.get_ent(ClassId::CObjectSentrygun) {
            let Some(sentry) = Entity::get_ent(id) else {continue};
            if vmt_call!(sentry.as_networkable(), is_dormant) {
                continue;
            }
            if sentry as *const _ == p_local.as_ent() as *const _ || !vmt_call!(sentry, is_alive) {
                continue;
            }
            let team = vmt_call!(sentry, get_team_number);

            for hitbox_id in sentry.as_object().unwrap().as_sentry().unwrap().get_hitbox_ids() {
                let (r, g, b) = hex_to_rgb!(team.color());
                let color = rgb_to_hex!(
                    r as f32 * COLOR_SCALE,
                    g as f32 * COLOR_SCALE,
                    b as f32 * COLOR_SCALE
                );
                let hitbox = sentry.get_hitbox(hitbox_id).unwrap();
                self.draw_hitbox(&sentry, hitbox, color, 50)?;
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

            let hitbox = pipe
                .get_hitbox(HitboxId::Head)
                .unwrap();
                //.scaled(HITBOX_SCALE);
            self.draw_hitbox(&pipe, hitbox, team.color(), 10)?;
        }
        Ok(())
    }
    pub fn draw_hitbox(
        &mut self,
        ent: &Entity,
        hitbox: Hitbox,
        color: usize,
        alpha: u8,
    ) -> OxideResult<()> {
        let corners = hitbox.corners(ent)?;
        let corners = corners.iter().map(|x|world_to_screen(x)).collect::<Vec<_>>();

        let pairs = [
            (corners[0].clone(), corners[1].clone()),
            (corners[0].clone(), corners[2].clone()),
            (corners[0].clone(), corners[4].clone()),
            (corners[7].clone(), corners[3].clone()),
            (corners[7].clone(), corners[5].clone()),
            (corners[7].clone(), corners[6].clone()),
            (corners[2].clone(), corners[3].clone()),
            (corners[2].clone(), corners[6].clone()),
            (corners[1].clone(), corners[5].clone()),
            (corners[1].clone(), corners[3].clone()),
            (corners[4].clone(), corners[6].clone()),
            (corners[4].clone(), corners[5].clone()),
        ];

        for pair in pairs {
            let Some(pos1) = &pair.0 else {
                continue;
            };
            let Some(pos2) = &pair.1 else {
                continue;
            };
            let (r, g, b) = hex_to_rgb!(color);
            vmt_call!(
                interface!(surface),
                set_color,
                r as isize,
                g as isize,
                b as isize,
                alpha as isize
            );

            vmt_call!(
                interface!(surface),
                draw_line,
                pos1.x as isize,
                pos1.y as isize,
                pos2.x as isize,
                pos2.y as isize
            );
        }
        Ok(())
    }
}
