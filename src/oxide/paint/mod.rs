use std::{collections::HashMap, ffi::CString, fs::File, io::Write};

use crate::{
    draw::fonts::HACK_FONT,
    error::OxideResult,
    hex_to_rgb, interface,
    math::{vector3::Vector3, view_matrix::VMatrix},
    o,
    sdk::font::{Font, FontFlags},
    vmt_call,
};

use self::frame::PaintFrame;

use super::interfaces::Interfaces;

pub mod esp;
pub mod hitbox;
pub mod frame;
pub mod explosives;

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
        let frame = PaintFrame{vmatrix: VMatrix::default()};
        self.draw_hitboxes(&frame)?;
        self.esp(&frame)?;
        self.draw_debug();
        self.explosives(&frame)?;
        Ok(())
    }
    pub fn draw_debug(&mut self) {
        for line in self.debug_lines.values() {
            let v_matrix = VMatrix::default();
            let Some(start) = v_matrix.world_to_screen(&line.start) else{return};
            let Some(end) = v_matrix.world_to_screen(&line.end) else {return};
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
}
