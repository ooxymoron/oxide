use std::{collections::HashMap, mem::MaybeUninit, ptr::null, usize};

use freetype_sys::*;
use libc::c_void;
use sdl2_sys::{
    SDL_BlendMode, SDL_CreateRGBSurfaceFrom, SDL_CreateTextureFromSurface, SDL_DestroyTexture,
    SDL_FreeSurface, SDL_Rect, SDL_RenderCopy, SDL_SetSurfaceBlendMode, SDL_Texture,
};

use crate::{d, hex_to_rgb};

pub static HACK_FONT: &[u8; 2215536] = include_bytes!("./../../assets/HackNerdFont-Regular.ttf");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CacheKey {
    letter: char,
    size: FontSize,
}

#[derive(Debug, Clone)]
struct CacheValue {
    texture: *mut SDL_Texture,
    width: i32,
    rows: i32,
    top: i32,
    left: i32,
    advance: FT_Vector,
}

#[derive(Debug, Clone)]
pub struct Fonts {
    pub free_type: FT_Library,
    pub face_large: FT_Face,
    pub face_medium: FT_Face,
    pub face_small: FT_Face,
    cache: HashMap<CacheKey, CacheValue>,
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub enum FontSize {
    Small = 16,
    Medium = 24,
    Large = 36,
}

impl Fonts {
    pub fn init_face(free_type: *mut c_void, size: isize) -> FT_Face {
        unsafe {
            let mut face = MaybeUninit::zeroed().assume_init();
            FT_New_Memory_Face(
                free_type,
                HACK_FONT.as_ptr(),
                HACK_FONT.len() as i64,
                0,
                &mut face,
            );
            let size = ((size) << 6) as i64;
            let res = 72;
            FT_Set_Char_Size(face, size, size, res, res);
            face
        }
    }
    pub fn init() -> Fonts {
        unsafe {
            let mut free_type = MaybeUninit::zeroed().assume_init();

            FT_Init_FreeType(&mut free_type);

            let face_large = Fonts::init_face(free_type, FontSize::Large as isize);
            let face_medium = Fonts::init_face(free_type, FontSize::Medium as isize);
            let face_small = Fonts::init_face(free_type, FontSize::Small as isize);
            Fonts {
                free_type,
                face_large,
                face_medium,
                face_small,
                cache: HashMap::new(),
            }
        }
    }
    pub fn get_face(&self, size: &FontSize) -> *mut FT_FaceRec {
        match size {
            FontSize::Small => self.face_small,
            FontSize::Medium => self.face_medium,
            FontSize::Large => self.face_large,
        }
    }
    pub fn restore(&self) {
        unsafe {
            FT_Done_Face(self.face_small);
            FT_Done_Face(self.face_medium);
            FT_Done_Face(self.face_large);
            FT_Done_FreeType(self.free_type);
        }
    }
    pub fn get_text_size(&mut self, text: &str, size: FontSize) -> (isize, isize, isize) {
        let face = self.get_face(&size);

        let mut w = 0;
        let mut h_min = 0;
        let mut h_max = 0;

        for letter in text.chars() {
            unsafe {
                FT_Load_Char(face, letter as u64, FT_LOAD_RENDER);

                let glyph = (*face).glyph.read_volatile();
                w += (glyph.metrics.horiAdvance >> 6) as isize;

                h_min = std::cmp::max((glyph.metrics.horiBearingY >> 6) as isize, h_min);
                h_max = std::cmp::max((glyph.metrics.horiBearingX >> 6) as isize, h_max);
            }
        }
        (w, h_min , h_max )
    }
    pub fn get_glyph(&self, size: FontSize, char: char) -> FT_GlyphSlotRec {
        let face = self.get_face(&size);

        unsafe {
            FT_Load_Char(face, char as u64, FT_LOAD_RENDER);
            (*face).glyph.read_volatile()
        }
    }
    pub fn draw_glyph(
        &mut self,
        face: &FT_Face,
        letter: char,
        size: FontSize,
        x: isize,
        y: isize,
        color: usize,
        alpha: u8,
    ) -> (isize, isize) {
        unsafe {

            if letter == ' ' {
                let size = self.get_text_size("a", FontSize::Small);
                return (size.0,0);
            }

            let cache_key = CacheKey { letter, size };
            if let Some(cached) = self.cache.get(&cache_key) {
                let mut rect = SDL_Rect {
                    x: x as i32 + cached.left,
                    y: y as i32 - cached.top,
                    w: cached.width,
                    h: cached.rows,
                };
                SDL_RenderCopy(d!().renderer, cached.texture.clone(), null(), &mut rect);
                return (
                    (cached.advance.x >> 6) as isize,
                    (cached.advance.y >> 6) as isize,
                );
            }

            let glyph_index = FT_Get_Char_Index(face.clone(), letter as u64);

            //TODO: ftload color?
            let error = FT_Load_Glyph(face.clone(), glyph_index, FT_LOAD_DEFAULT);
            if error != 0 {
                return (0, 0);
            }

            let error = FT_Render_Glyph(face.read().glyph, FT_RENDER_MODE_NORMAL);
            if error != 0 {
                return (0, 0);
            }
            let bitmap = face.read().glyph.read().bitmap;

            let len = (bitmap.width * bitmap.rows * 4) as usize;
            if len == 0 {
                return (0, 0);
            }

            let mut rgba = vec![0u8; len];

            let buffer = std::slice::from_raw_parts(bitmap.buffer, len);
            let color = hex_to_rgb!(color);
            for i in (0..len).step_by(4) {
                let val = buffer[i / 4];

                (rgba[i], rgba[i + 1], rgba[i + 2]) = color;
                rgba[i + 3] = (val as f32 * (alpha as f32 / 255f32)) as u8;
            }

            let surface = SDL_CreateRGBSurfaceFrom(
                rgba.as_ptr() as *mut c_void,
                bitmap.width,
                bitmap.rows,
                32,
                bitmap.width * 4,
                0x000000ff,
                0x0000ff00,
                0x00ff0000,
                0xff000000,
            );

            SDL_SetSurfaceBlendMode(surface, SDL_BlendMode::SDL_BLENDMODE_BLEND);

            let texture = SDL_CreateTextureFromSurface(d!().renderer, surface);

            let slot = face.read().glyph.read();
            let mut rect = SDL_Rect {
                x: x as i32 + slot.bitmap_left,
                y: y as i32 - slot.bitmap_top,
                w: bitmap.width,
                h: bitmap.rows,
            };
            SDL_RenderCopy(d!().renderer, texture, null(), &mut rect);
            self.cache.insert(
                cache_key,
                CacheValue {
                    texture,
                    width: bitmap.width,
                    rows: bitmap.rows,
                    top: slot.bitmap_top,
                    left: slot.bitmap_left,
                    advance: slot.advance,
                },
            );
            SDL_FreeSurface(surface);
            return (
                (slot.advance.x >> 6) as isize,
                (slot.advance.y >> 6) as isize,
            );
        }
    }
    pub fn resture(&mut self) {
        for val in self.cache.clone().into_values() {
            unsafe {
                SDL_DestroyTexture(val.texture);
            }
        }
    }
}
