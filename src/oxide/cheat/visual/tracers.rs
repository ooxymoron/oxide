use std::{mem::transmute, usize};

use crate::{
    draw::colors::{GREEN, LIGHT_BLUE, LIGHT_RED, WHITE},
    interface,
    math::angles::Angles,
    sdk::{
        entity::{player::Player, weapon::Weapon},
        fire_bullets_info::FireBulletsInfo,
        interfaces::{
            engine_trace::{trace, TraceFilter, CONTENTS_GRATE, MASK_SHOT},
            entity::{hitbox::PlayerHitboxId, Entity},
        },
        networkable::ClassId,
    },
    setting, vmt_call,
};

use super::Visuals;

impl Visuals {
    pub fn draw_fire_tracer(&self, angles: &Angles) {
        let p_local = Player::get_local().unwrap();
        let weapon = vmt_call!(p_local.as_ent(), get_weapon);
        let range = weapon.get_info().weapon_data[weapon.get_mode()].range;
        let dir = angles.to_vectors().forward * range;
        let src = vmt_call!(p_local.as_ent(), eye_position);
        let filter = TraceFilter::new(p_local.as_ent());
        let trace = trace(&src, &(src + dir), MASK_SHOT | CONTENTS_GRATE, &filter);
        let color = WHITE;
        let alpha = 20;
        let time = 0.5;
        if *setting!(visual, impacts) {
            interface!(debug_overlay).rect(&trace.endpos, 4.0, color, alpha, time);
            interface!(debug_overlay).triangle(&src, 4.0, color, alpha, time);
        }
        if *setting!(visual, tracers) {
            interface!(debug_overlay).line(
                &trace.startpos,
                &trace.endpos.clone(),
                color,
                alpha,
                time,
            );
        }
    }
    pub fn draw_bullet_tracer(&self, info: &FireBulletsInfo, weapon: *mut Weapon) {
        if (!*setting!(visual, tracers) && !*setting!(visual, impacts)) || weapon.is_null() {
            return;
        }
        let p_local = Player::get_local().unwrap();
        let weapon = unsafe { transmute::<_, &mut Weapon>(weapon) };
        let Ok(plocal) = Player::get_local() else { return };
        let end = info.src + info.dir_shooting * info.distance;
        let filter = TraceFilter::new(p_local.as_ent());
        let trace = trace(&info.src, &end, MASK_SHOT | CONTENTS_GRATE, &filter);
        let mut time = 0.5;
        let mut color = LIGHT_BLUE;
        let mut alpha = 10;
        if !trace.entity.is_null() {
            let target = unsafe { transmute::<_, &Entity>(trace.entity) };

            match target.as_networkable().get_client_class().class_id {
                ClassId::CTFPlayer
                | ClassId::CObjectSentrygun
                | ClassId::CObjectDispenser
                | ClassId::CObjectTeleporter => {
                    let target_team = vmt_call!(target, get_team_number);
                    let my_team = vmt_call!(plocal.as_ent(), get_team_number);
                    if target_team != my_team {
                        time = 2.0;
                        color = LIGHT_RED;
                        alpha = 30;
                        if trace.hitbox_id == PlayerHitboxId::Head as usize && weapon.can_headshot()
                        {
                            color = GREEN;
                        }
                    }
                }
                _ => {}
            }
        }
        if *setting!(visual, impacts) {
            interface!(debug_overlay).rect(&trace.endpos, 4.0, color, alpha, time);
        }
        if *setting!(visual, tracers) {
            interface!(debug_overlay).line(
                &trace.startpos,
                &trace.endpos.clone(),
                color,
                alpha,
                time,
            );
        }
    }
}
