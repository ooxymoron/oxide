use crate::{
    draw::colors::{BLUE, FOREGROUND, FOREGROUND3, GREEN},
    hex_to_rgb, interface,
    math::{get_corners, vector::Vector2},
    o,
    util::world_to_screen,
    vmt_call,
};

use super::Entity;

const PAD: i32 = 5;

impl Entity {
    pub fn paint(
        &self,
        r#box: bool,
        draw_hp: bool,
        text_top: Option<&str>,
        text_right: Vec<String>,
    ) {
        let team = vmt_call!(self, get_team_number);
        let collidable = vmt_call!(self, get_collideable);
        let min = *vmt_call!(collidable, obb_mins);
        let max = *vmt_call!(collidable, obb_maxs);
        let origin = *vmt_call!(collidable, get_origin);
        let angles = *vmt_call!(collidable, get_angles);
        let corners = get_corners(&origin, &angles, &min, &max);
        let corners = corners
            .iter()
            .filter_map(|corner| world_to_screen(corner))
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
            let (r, g, b) = hex_to_rgb!(GREEN);
            vmt_call!(interface!(surface), set_color, r, g, b, 50);
            let health = vmt_call!(self, get_health);
            let max_health = vmt_call!(self, get_max_health);
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
        }
        if let Some(text) = text_top {
            o!().paint.paint_text(
                &text,
                ((minx + maxx) / 2.0) as i32,
                (miny - PAD as f32) as i32,
                FOREGROUND,
                true,
            );
        }
        let mut y = miny as i32;
        for text in text_right {
            o!().paint
                .paint_text(&text, (maxx + PAD as f32) as i32, y, FOREGROUND3, false);
            y += o!().paint.get_text_size(&text).1;
        }
    }
}
