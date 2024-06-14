use std::{collections::HashMap, mem::MaybeUninit, ptr::null, usize};

use freetype_sys::*;
use libc::c_void;
use sdl2_sys::{
    SDL_BlendMode, SDL_CreateRGBSurfaceFrom, SDL_CreateTextureFromSurface, SDL_DestroyTexture,
    SDL_FreeSurface, SDL_Rect, SDL_RenderCopy, SDL_SetSurfaceBlendMode, SDL_Texture,
};

use crate::{d, hex_to_rgb, log};

pub static HACK_FONT: &[u8; 2215536] = include_bytes!("./../../assets/HackNerdFont-Regular.ttf");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CacheKey {
    text: String,
    size: FontSize,
    color: usize,
    alpha: u8,
}

#[derive(Debug, Clone)]
struct CacheValue {
    texture: *mut SDL_Texture,
    width: i32,
    rows: i32,
    top: i32,
    left: i32,
}

#[derive(Debug, Clone)]
pub struct Fonts {
    pub free_type: FT_Library,
    pub faces: HashMap<FontSize, FT_Face>,
    cache: HashMap<CacheKey, CacheValue>,
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
#[repr(isize)]
pub enum FontSize {
    Small = 8,
    Medium = 16,
    Large = 24,
    Huge = 36,
}

impl FontSize {
    pub fn height(&self) -> isize {
        self.clone() as isize
    }
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

            let mut faces = HashMap::new();

            macro_rules! init_face {
                ($size: path) => {
                    faces.insert($size, Fonts::init_face(free_type, $size as isize));
                };
            }
            init_face!(FontSize::Small);
            init_face!(FontSize::Medium);
            init_face!(FontSize::Large);
            init_face!(FontSize::Huge);
            Fonts {
                free_type,
                faces,
                cache: HashMap::new(),
            }
        }
    }
    pub fn restore(&self) {
        unsafe {
            for face in self.faces.values() {
                FT_Done_Face(*face);
            }
            FT_Done_FreeType(self.free_type);
        }
    }
    pub fn get_text_size(&mut self, text: &str, size: FontSize) -> (isize, isize, isize) {
        let face = *self.faces.get(&size).unwrap();

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
        (w, h_min, h_max)
    }
    pub fn get_glyph(&self, size: FontSize, char: char) -> FT_GlyphSlotRec {
        let face = *self.faces.get(&size).unwrap();

        unsafe {
            FT_Load_Char(face, char as u64, FT_LOAD_RENDER);
            (*face).glyph.read_volatile()
        }
    }
    pub fn draw_text(
        &mut self,
        text: &str,
        size: FontSize,
        x: isize,
        y: isize,
        color: usize,
        alpha: u8,
    ) {
        unsafe {
            let cache_key = CacheKey {
                text: text.to_string(),
                size: size.clone(),
                color,
                alpha,
            };
            if let Some(cached) = self.cache.get(&cache_key) {
                let mut rect = SDL_Rect {
                    x: x as i32 + cached.left,
                    y: y as i32 - cached.top,
                    w: cached.width,
                    h: cached.rows,
                };
                SDL_RenderCopy(d!().renderer, cached.texture.clone(), null(), &mut rect);
                return;
            }
            log!("caching text texture {}", text);

            let face = self.faces.get(&size).unwrap();

            let mut bitmaps = Vec::new();

            for letter in text.chars() {
                let glyph_index = FT_Get_Char_Index(face.clone(), letter as u64);

                FT_Load_Glyph(face.clone(), glyph_index, FT_LOAD_DEFAULT);

                FT_Render_Glyph(face.read().glyph, FT_RENDER_MODE_NORMAL);
                let bitmap = face.read().glyph.read().bitmap;

                let width = bitmap.width as usize;
                let height = bitmap.rows as usize;

                if bitmap.buffer.is_null() {
                    let w = size.height() as usize;
                    bitmaps.push((Vec::new(), w, 0, w, 0, 0));
                    continue;
                }
                let buffer = std::slice::from_raw_parts(bitmap.buffer, width * height);

                let mut vec_bitmap = vec![vec![0u8; width]; height];

                for row in 0..height {
                    for coll in 0..width {
                        vec_bitmap[row][coll] =
                            (buffer[row * width + coll] as f32 * (alpha as f32 / 255f32)) as u8;
                    }
                }
                let bearing_y = (face.read().glyph.read().metrics.horiBearingY >> 6) as usize;
                let bearing_x = (face.read().glyph.read().metrics.horiBearingX >> 6) as usize;
                let advance = (face.read().glyph.read().metrics.horiAdvance >> 6) as usize;
                bitmaps.push((
                    vec_bitmap,
                    width,
                    height,
                    advance,
                    bearing_y,
                    bearing_x,
                ));
            }

            let width = bitmaps.iter().fold(0, |acc, x| acc + x.3);
            let y_origin_offset = bitmaps.iter().fold(0, |acc, x| acc.max(x.4));
            let height = (face.read().max_advance_height >> 6) as usize;

            let mut rgba_bitmap = vec![0u8; (width * height as usize) * 4];

            let color = hex_to_rgb!(color);
            for row_i in 0..height {
                let mut x_offset = 0;
                for bitmap in &bitmaps {
                    if row_i >= bitmap.2 {
                        x_offset += bitmap.3;
                        continue;
                    }
                    for cell_i in 0..bitmap.1 {
                        let i =
                            ((row_i + y_origin_offset - bitmap.4) * width + cell_i + x_offset + bitmap.5) * 4;
                        (rgba_bitmap[i], rgba_bitmap[i + 1], rgba_bitmap[i + 2]) = color;
                        rgba_bitmap[i + 3] = bitmap.0[row_i][cell_i]
                    }
                    x_offset += bitmap.3;
                }
            }

            let surface = SDL_CreateRGBSurfaceFrom(
                rgba_bitmap.as_ptr() as *mut c_void,
                width as i32,
                height as i32,
                32,
                width as i32 * 4,
                0x000000ff,
                0x0000ff00,
                0x00ff0000,
                0xff000000,
            );

            SDL_SetSurfaceBlendMode(surface, SDL_BlendMode::SDL_BLENDMODE_BLEND);
            let texture = SDL_CreateTextureFromSurface(d!().renderer, surface);
            let mut rect = SDL_Rect {
                x: x as i32,
                y: y as i32 + y_origin_offset as i32 ,
                w: width as i32,
                h: height as i32,
            };
            SDL_RenderCopy(d!().renderer, texture, null(), &mut rect);
            self.cache.insert(
                cache_key,
                CacheValue {
                    texture,
                    width: width as i32,
                    rows: height as i32,
                    top: y_origin_offset as i32 ,
                    left: 0,
                },
            );
            SDL_FreeSurface(surface);
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
