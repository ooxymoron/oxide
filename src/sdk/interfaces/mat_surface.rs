use std::os::raw::c_char;

use self::font::{FontDrawType, HFont};

use super::*;


pub type Surface = WithVmt<VMTMatSurface>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTMatSurface {
    _pad1: [u64; 10],
    pub set_color: cfn!((), &'static Surface, i32, i32, i32, i32),
    _pad2: [u64; 1],
    pub draw_filled_rect: cfn!((), &'static Surface, i32, i32, i32, i32),
    _pad3: [u64; 1],
    pub draw_rect: cfn!((), &'static Surface, i32, i32, i32, i32),
    pub draw_line: cfn!((), &'static Surface, i32, i32, i32, i32),
    _pad4: [u64; 1],
    pub set_text_font: cfn!((), &'static Surface, HFont),
    pub set_text_color: cfn!((), &'static Surface, u8, u8, u8, u8),
    _pad5: [u64; 1],
    pub set_text_pos: cfn!((), &'static Surface, i32, i32),
    pub get_text_pos: cfn!((), &'static Surface, i32, i32),
    pub print_text: cfn!((), &'static Surface, *const u32, u32, FontDrawType),
    _pad6: [u64; 29],
    pub set_cursor_always_visible: cfn!((), &'static Surface, bool),
    pub is_cursor_visible: cfn!(bool, &'static Surface),
    pub apply_changes: cfn!((), &'static Surface),
    _pad71: [u64; 6],
    pub unlock_cursor: cfn!((), &'static Surface),
    pub lock_cursor: cfn!((), &'static Surface),
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
        (),
        &Surface,
        HFont,
        *const u32,
        &mut i32,
        &mut i32
    ),
    _pad81: [u64; 2],
    pub play_sound: cfn!((), &'static Surface, *const c_char),
    //virtual void PlaySound(const char *fileName) = 0;
    _pad9: [u64; 20],
    pub draw_circle: cfn!((), &'static Surface, i32, i32, i32, i32),
    _pad10: [u64; 11],
    pub on_screen_size_changed: cfn!((), &'static Surface, i32, i32),
}
