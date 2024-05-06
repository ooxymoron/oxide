use std::{collections::HashMap, ffi::CString, fs::File, io::Write};

use crate::{
    draw::fonts::HACK_FONT,
    error::OxideResult,
    hex_to_rgb, interface,
    math::vector::Vector3,
    o,
    sdk::font::{Font, FontDrawType, FontFlags},
    util::world_to_screen,
    vmt_call,
};

use super::interfaces::Interfaces;

pub mod esp;
pub mod hitbox;

#[derive(Debug)]
pub struct Paint {
    pub normal: Font,
    pub debug_lines: HashMap<String, DebugLine>,
}
#[derive(Debug)]
pub struct DebugLine {
    pub start: Vector3,
    pub end: Vector3,
    pub color: usize,
}

impl Paint {
    pub fn init(interfaces: &Interfaces) -> Paint {
        let surface = interfaces.surface.interface_ref();

        let file_name = "HackNerdFont-Regular.ttf";

        let mut file = File::create(file_name).unwrap();
        file.write_all(HACK_FONT).unwrap();

        let name = CString::new("Hack").unwrap();
        let path = CString::new(file_name).unwrap();

        vmt_call!(surface, add_custom_font_file, name.as_ptr(), path.as_ptr());

        let id = vmt_call!(surface, create_font);

        let normal = Font {
            name: name.as_ptr(),
            tall: 16,
            weight: 700,
            flags: FontFlags::OUTLINE as i32,
            id,
        };

        vmt_call!(
            surface,
            set_font_glyph_set,
            normal.id,
            normal.name,
            normal.tall,
            normal.weight,
            0,
            0,
            normal.flags,
            0,
            0
        );

        Paint {
            normal,
            debug_lines: HashMap::new(),
        }
    }
    pub fn paint(&mut self) -> OxideResult<()> {
        if let Some(cache) = &o!().last_entity_cache {
            self.draw_hitboxes(&cache)?;
            self.esp(&cache)?;
        }
        self.draw_debug();
        Ok(())
    }
    pub fn draw_debug(&mut self) {
        for line in self.debug_lines.values() {
            let Some(start) = world_to_screen(&line.start) else{return};
            let Some(end) = world_to_screen(&line.end) else {return};
            let (r, g, b) = hex_to_rgb!(line.color);
            vmt_call!(interface!(surface), set_color, r, g, b, 255);
            vmt_call!(
                interface!(surface),
                draw_line,
                start.x as i32,
                start.y as i32,
                end.x as i32,
                end.y as i32
            );
        }
    }
    pub fn get_text_size(&mut self, text: &str) -> (i32, i32) {
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
    pub fn paint_text(&mut self, text: &str, mut x: i32, mut y: i32, color: usize, center: bool) {
        let mut text = text.to_owned();
        if text.bytes().last() != Some(0) {
            text += "\0";
        }
        let char_text = text
            .chars()
            .into_iter()
            .map(|x| x as u32)
            .collect::<Vec<_>>();

        if center {
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
            x -= w / 2;
            y -= h / 2;
        }

        vmt_call!(interface!(surface), set_text_font, o!().paint.normal.id);
        vmt_call!(interface!(surface), set_text_pos, x, y);
        let (r, g, b) = hex_to_rgb!(color);
        vmt_call!(interface!(surface), set_text_color, r, g, b, 255);
        vmt_call!(
            interface!(surface),
            print_text,
            char_text.as_ptr(),
            char_text.len() as u32,
            FontDrawType::Default
        );
    }
}
