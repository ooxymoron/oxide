use std::mem::transmute;

use crate::{
    draw::colors::{GREEN, LIGHT_BLUE, LIGHT_RED},
    interface,
    sdk::{
        entity::{player::Player, weapon::Weapon},
        fire_bullets_info::FireBulletsInfo,
        interfaces::{
            engine_trace::{trace, CONTENTS_GRATE, MASK_SHOT},
            entity::Entity,
            model_info::HitboxId,
        },
        networkable::ClassId,
    },
    setting, vmt_call,
};

use super::Visuals;

impl Visuals {
    pub fn draw_tracer(&self, info: &FireBulletsInfo, weapon: *mut Weapon) {
        if (!setting!(visual, tracers) && !setting!(visual, impacts)) || weapon.is_null(){
            return;
        }
        let weapon = unsafe{transmute::<_,&mut Weapon>(weapon)};
        let Ok(plocal) = Player::get_local() else { return };
        let end = info.src + info.dir_shooting * info.distance;
        let trace = trace(info.src, end, MASK_SHOT | CONTENTS_GRATE);
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
                        if trace.hitbox_id == HitboxId::Head && weapon.can_headshot() {
                            color = GREEN;
                        }
                    }
                }
                _ => {}
            }
        }
        if setting!(visual, impacts) {
            interface!(debug_overlay).rect(&trace.endpos, 4.0, color, alpha, time);
        }
        if setting!(visual, tracers) {
            interface!(debug_overlay).line(&trace.startpos, &trace.endpos.clone(), color, alpha, time);
        }
    }
}
