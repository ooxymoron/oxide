use crate::{
    hex_to_rgb, interface, math::view_matrix::VMatrix, o, sdk::font::FontDrawType, vmt_call,
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
                y += h / 2;
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
}
