use libc::c_void;

use self::font::{FontDrawType, HFont};

use super::*;


pub type Surface = WithVmt<VMTMatSurface>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTMatSurface {
    _pad1: [u64; 10],
    pub set_color: cfn!((), &'static Surface, isize, isize, isize, isize),
    _pad2: [u64; 1],
    pub draw_filled_rect: cfn!((), &'static Surface, isize, isize, isize, isize),
    _pad3: [u64; 1],
    pub draw_rect: cfn!((), &'static Surface, isize, isize, isize, isize),
    pub draw_line: cfn!((), &'static Surface, isize, isize, isize, isize),
    _pad4: [u64; 1],
    pub set_text_font: cfn!((), &'static Surface, HFont),
    pub set_text_color: cfn!((), &'static Surface, u8, u8, u8, u8),
    _pad5: [u64; 1],
    pub set_text_pos: cfn!((), &'static Surface, isize, isize),
    pub get_text_pos: cfn!((), &'static Surface, isize, isize),
    pub print_text: cfn!((), &'static Surface, *const u32, usize, FontDrawType),
    _pad6: [u64; 29],
    pub set_cursor_always_visible: cfn!((), &'static Surface, bool),
    pub id_cursor_visible: cfn!(bool, &'static Surface),
    _pad71: [u64; 7],
    pub lock_cursor: cfn!((), &'static Surface),
    pub unlock_cursor: cfn!((), &'static Surface),
    _pad7: [u64; 3],
    pub create_font: cfn!(HFont, &'static Surface),
    pub set_font_glyph_set: cfn!(
        bool,
        &'static Surface,
        HFont,
        *const i8,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32
    ),
    pub add_custom_font_file: cfn!(bool, &Surface, *const i8, *const i8),
    _pad8: [u64; 6],
    pub get_text_size: cfn!(
        c_void,
        &Surface,
        HFont,
        *const u32,
        &mut isize,
        &mut isize
    ),
    _pad9: [u64; 23],
    pub draw_circle: cfn!(c_void, &'static Surface, isize, isize, isize, isize),
    _pad10: [u64; 11],
    pub on_screen_size_changed: cfn!(c_void, &'static Surface, isize, isize),
}
