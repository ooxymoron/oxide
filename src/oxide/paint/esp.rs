use crate::{
    draw::colors::{BACKGROUND, BACKGROUND2, BLUE, FOREGROUND, FOREGROUND3, GREEN},
    error::OxideResult,
    hex_to_rgb, interface,
    math::{get_corners, vector2::Vector2},
    o,
    sdk::{
        condition::ConditionFlags,
        entity::{player::Player, Entity},
        networkable::ClassId,
    },
    setting, vmt_call,
};

use super::{frame::PaintFrame, Paint};
const PAD: i32 = 5;
const CONDITIONS: [ConditionFlags; 19] = [
    ConditionFlags::Ubercharged,
    ConditionFlags::Bonked,
    ConditionFlags::Aiming,
    ConditionFlags::Disguised,
    ConditionFlags::Cloaked,
    ConditionFlags::Taunting,
    ConditionFlags::DeadRingered,
    ConditionFlags::Dazed,
    ConditionFlags::Charging,
    ConditionFlags::CritCola,
    ConditionFlags::Jarated,
    ConditionFlags::DefenseBuffed,
    ConditionFlags::Buffed,
    ConditionFlags::Milked,
    ConditionFlags::MarkedForDeath,
    ConditionFlags::RegenBuffed,
    ConditionFlags::UberBulletResist,
    ConditionFlags::UberBlastResist,
    ConditionFlags::UberFireResist,
];

impl Paint {
    pub fn esp(&mut self, frame: &PaintFrame) -> OxideResult<()> {
        if !vmt_call!(interface!(base_engine), is_in_game) || !setting!(visual, esp) {
            return Ok(());
        }
        let Some(cache) = o!().last_entity_cache.as_ref() else {return Ok(())};
        let p_local = Player::get_local()?;
        for id in cache.get_ent(ClassId::CTFPlayer) {
            let Some(ent) = Entity::get_ent(id) else {
                continue;
            };
            if vmt_call!(ent.as_networkable(), is_dormant) {
                continue;
            }
            if ent as *const _ == p_local.as_ent() as *const _ || !vmt_call!(ent, is_alive) {
                continue;
            }
            if !setting!(visual, esp_friendlies)
                && vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number)
            {
                continue;
            }

            let player = ent.as_player()?;
            let player_cond = player.get_condition();
            let conditions = CONDITIONS
                .iter()
                .filter_map(|&cond| {
                    if !player_cond.get(cond) {
                        return None;
                    }
                    return Some(format!("{:?}", cond));
                })
                .collect::<Vec<_>>();

            let info = player.info()?;
            let name = info.name;
            self.paint_esp_box(frame, ent, true, true, Some(&name), conditions);
        }
        if setting!(visual, esp_sentreis) {
            for id in cache.get_ent(ClassId::CObjectSentrygun) {
                let Some(ent) = Entity::get_ent(id) else{
                    continue;
                };
                let obj = ent.as_object()?;
                if vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number)
                    && !setting!(visual, esp_friendlies)
                    || *obj.get_carried()
                {
                    continue;
                }
                let text = if *obj.get_mini() {
                    vec!["MINI".to_owned()]
                } else {
                    vec![format!("LEVEL: {:?}", obj.get_level())]
                };
                self.paint_esp_box(frame, ent, true, true, Some("sentry"), text);
            }
        }

        if setting!(visual, esp_projectiles) {
            for id in cache.get_ent(ClassId::CTFProjectileRocket) {
                let Some(ent) = Entity::get_ent(id) else{
                    continue;
                };
                if vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number)
                    && !setting!(visual, esp_friendlies)
                {
                    continue;
                }
                self.paint_esp_box(frame, ent, false, false, Some("rocket"), vec![]);
            }
            for id in cache.get_ent(ClassId::CTFGrenadePipebombProjectile) {
                let Some(ent) = Entity::get_ent(id) else{
                    continue;
                };
                if vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number)
                    && !setting!(visual, esp_friendlies)
                {
                    continue;
                }
                let text = ent.as_pipe()?.get_type().to_str();
                self.paint_esp_box(frame, ent, false, false, Some(text), vec![]);
            }
        }

        Ok(())
    }
    pub fn paint_esp_box(
        &self,
        frame: &PaintFrame,
        ent: &Entity,
        r#box: bool,
        draw_hp: bool,
        text_top: Option<&str>,
        text_right: Vec<String>,
    ) {
        let team = vmt_call!(ent, get_team_number);
        let collidable = vmt_call!(ent, get_collideable);
        let min = *vmt_call!(collidable, obb_mins);
        let max = *vmt_call!(collidable, obb_maxs);
        let origin = *vmt_call!(collidable, get_origin);
        let angles = *vmt_call!(collidable, get_angles);
        let corners = get_corners(&origin, &angles.to_vectors(), &min, &max);

        let corners = corners
            .iter()
            .filter_map(|corner| frame.vmatrix.world_to_screen(corner))
            .collect::<Vec<_>>();
        if corners.is_empty() {
            return;
        }
        let mut minx = None;
        let mut maxx = None;
        let mut miny = None;
        let mut maxy = None;
        for Vector2 { x, y } in corners {
            if if let Some(val) = minx { val > x } else { true } {
                minx = Some(x)
            }
            if if let Some(val) = maxx { val < x } else { true } {
                maxx = Some(x)
            }
            if if let Some(val) = miny { val > y } else { true } {
                miny = Some(y)
            }
            if if let Some(val) = maxy { val < y } else { true } {
                maxy = Some(y)
            }
        }
        let minx = minx.unwrap();
        let maxx = maxx.unwrap();
        let miny = miny.unwrap();
        let maxy = maxy.unwrap();

        if r#box {
            let (r, g, b) = hex_to_rgb!(team.color());
            vmt_call!(interface!(surface), set_color, r, g, b, 50);
            vmt_call!(
                interface!(surface),
                draw_rect,
                minx as i32,
                miny as i32,
                maxx as i32,
                maxy as i32
            );
        }

        if draw_hp {
            let health = vmt_call!(ent, get_health);
            let max_health = vmt_call!(ent, get_max_health);
            let (r, g, b) = hex_to_rgb!(BACKGROUND);
            vmt_call!(interface!(surface), set_color, r, g, b, 50);
            vmt_call!(
                interface!(surface),
                draw_filled_rect,
                minx as i32 - 2 * PAD,
                miny as i32,
                minx as i32 - PAD,
                maxy as i32
            );
            let (r, g, b) = hex_to_rgb!(GREEN);
            vmt_call!(interface!(surface), set_color, r, g, b, 50);
            vmt_call!(
                interface!(surface),
                draw_filled_rect,
                minx as i32 - 2 * PAD,
                miny as i32
                    + ((1.0 - (health.min(max_health) as f32 / max_health as f32))
                        * (maxy as f32 - miny as f32)) as i32,
                minx as i32 - PAD,
                maxy as i32
            );
            if health > max_health {
                let (r, g, b) = hex_to_rgb!(BLUE);
                vmt_call!(interface!(surface), set_color, r, g, b, 50);
                vmt_call!(
                    interface!(surface),
                    draw_filled_rect,
                    minx as i32 - 2 * PAD,
                    miny as i32
                        + ((1.0 - ((health - max_health) as f32 / max_health as f32))
                            * (maxy as f32 - miny as f32)) as i32,
                    minx as i32 - PAD,
                    maxy as i32
                );
            }
            let (r, g, b) = hex_to_rgb!(BACKGROUND2);
            vmt_call!(interface!(surface), set_color, r, g, b, 50);
            vmt_call!(
                interface!(surface),
                draw_rect,
                minx as i32 - 2 * PAD,
                miny as i32,
                minx as i32 - PAD,
                maxy as i32
            );
        }
        if let Some(text) = text_top {
            frame.paint_text(
                &text,
                minx as i32,
                (miny - PAD as f32) as i32,
                FOREGROUND,
                255,
                false,
                true,
            );
        }
        let mut y = miny as i32;
        for text in text_right {
            frame.paint_text(
                &text,
                (maxx + PAD as f32) as i32,
                y,
                FOREGROUND3,
                255,
                false,
                false,
            );
            y += frame.get_text_size(&text).1;
        }
    }
}
