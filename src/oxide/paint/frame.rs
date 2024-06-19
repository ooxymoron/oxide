use crate::{
    hex_to_rgb, interface,
    math::{vector3::Vector3, view_matrix::VMatrix},
    o,
    sdk::font::FontDrawType,
    vmt_call,
};

#[derive(Debug)]
pub struct PaintFrame {
    pub vmatrix: VMatrix,
}
impl PaintFrame {
    pub fn get_text_size(&self, text: &str) -> (i32, i32) {
        let mut text = text.to_owned();
        if text.bytes().last() != Some(0) {
            text += "\0";
        }
        let char_text = text
            .chars()
            .into_iter()
            .map(|x| x as u32)
            .collect::<Vec<_>>();
        let mut w = 0;
        let mut h = 0;
        vmt_call!(
            interface!(surface),
            get_text_size,
            o!().paint.normal.id,
            char_text.as_ptr(),
            &mut w,
            &mut h
        );
        return (w, h);
    }
    pub fn paint_text(
        &self,
        text: &str,
        mut x: i32,
        mut y: i32,
        color: usize,
        alpha: u8,
        center_horizontaly: bool,
        center_vertically: bool,
    ) {
        let mut text = text.to_owned();
        if text.bytes().last() != Some(0) {
            text += "\0";
        }
        let char_text = text
            .chars()
            .into_iter()
            .map(|x| x as u32)
            .collect::<Vec<_>>();

        if center_horizontaly || center_vertically {
            let mut w = 0;
            let mut h = 0;
            vmt_call!(
                interface!(surface),
                get_text_size,
                o!().paint.normal.id,
                char_text.as_ptr(),
                &mut w,
                &mut h
            );
            if center_vertically {
                y -= h / 2;
            }
            if center_horizontaly {
                x -= w / 2;
            }
        }

        vmt_call!(interface!(surface), set_text_font, o!().paint.normal.id);
        vmt_call!(interface!(surface), set_text_pos, x, y);
        let (r, g, b) = hex_to_rgb!(color);
        vmt_call!(interface!(surface), set_text_color, r, g, b, alpha);
        vmt_call!(
            interface!(surface),
            print_text,
            char_text.as_ptr(),
            char_text.len() as u32,
            FontDrawType::Default
        );
    }
    pub fn triangle3d(&self, a: &Vector3, b: &Vector3, c: &Vector3, color: usize, alpha: u8) {
        let Some(a) = self.vmatrix.world_to_screen(a) else {return};
        let Some(b) = self.vmatrix.world_to_screen(b) else {return};
        let Some(c) = self.vmatrix.world_to_screen(c) else {return};
        {
            let (r, g, b) = hex_to_rgb!(color);
            vmt_call!(interface!(surface), set_color, r, g, b, alpha);
        }

        vmt_call!(
            interface!(surface),
            draw_line,
            a.x as i32,
            a.y as i32,
            b.x as i32,
            b.y as i32
        );

        vmt_call!(
            interface!(surface),
            draw_line,
            b.x as i32,
            b.y as i32,
            c.x as i32,
            c.y as i32
        );

        vmt_call!(
            interface!(surface),
            draw_line,
            c.x as i32,
            c.y as i32,
            a.x as i32,
            a.y as i32
        );
    }
    pub fn icosphere(&self, pos: Vector3, r: f32, color: usize, alpha: u8) {
        let t = ((1. + 5f32.sqrt()) / 2.).atan2(1.);
        let a = t.sin() * r;
        let b = t.cos() * r;
        let verts = [
            Vector3::new(-b, a, 0.),
            Vector3::new(b, a, 0.),
            Vector3::new(-b, -a, 0.),
            Vector3::new(b, -a, 0.),
            Vector3::new(0., -b, a),
            Vector3::new(0., b, a),
            Vector3::new(0., -b, -a),
            Vector3::new(0., b, -a),
            Vector3::new(a, 0., -b),
            Vector3::new(a, 0., b),
            Vector3::new(-a, 0., -b),
            Vector3::new(-a, 0., b),
        ]
        .map(|x| x + pos);

        macro_rules! t {
            ($a: expr, $b: expr, $c: expr) => {
                self.triangle3d(&verts[$a], &verts[$b], &verts[$c], color, alpha)
            };
        }

        t!(0, 11, 5);
        t!(0, 5, 1);
        t!(0, 1, 7);
        t!(0, 7, 10);
        t!(0, 10, 11);
        t!(1, 5, 9);
        t!(5, 11, 4);
        t!(11, 10, 2);
        t!(10, 7, 6);
        t!(7, 1, 8);
        t!(3, 9, 4);
        t!(3, 4, 2);
        t!(3, 2, 6);
        t!(3, 6, 8);
        t!(3, 8, 9);
        t!(4, 9, 5);
        t!(2, 4, 11);
        t!(6, 2, 10);
        t!(8, 6, 7);
        t!(9, 8, 1);
    }
}
