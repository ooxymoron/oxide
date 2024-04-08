use std::{f32::consts::PI, ptr::null};

use sdl2_sys::*;

use crate::{d, hex_to_rgb};

use super::{fonts::FontSize, frame::Frame};

pub static LOGO: &[u8; 1126522] = include_bytes!("../../oxide-logo.bmp");

impl Frame {
    pub fn filled_rect(&self, x: isize, y: isize, w: isize, h: isize, color: usize, alpha: u8) {
        let rect = SDL_Rect {
            x: x as i32,
            y: y as i32,
            w: w as i32,
            h: h as i32,
        };

        self.set_color(color, alpha);
        unsafe {
            SDL_RenderFillRect(self.renderer, &rect);
        }
    }

    pub fn logo(&self, x: isize, y: isize, w: isize, h: isize) {
        unsafe {
            let rect = SDL_Rect {
                x: x as i32,
                y: y as i32,
                w: w as i32,
                h: h as i32,
            };
            SDL_RenderCopy(d!().renderer, d!().logo, null(), &rect);
        }
    }

    pub fn outlined_rect(&self, x: isize, y: isize, w: isize, h: isize, color: usize, alpha: u8) {
        self.set_color(color, alpha);

        let rect = SDL_Rect {
            x: x as i32,
            y: y as i32,
            w: w as i32,
            h: h as i32,
        };
        unsafe {
            SDL_RenderDrawRect(self.renderer, &rect);
        }
    }

    pub fn line(&self, x1: isize, y1: isize, x2: isize, y2: isize, color: usize, alpha: u8) {
        self.set_color(color, alpha);

        unsafe {
            SDL_RenderDrawLine(self.renderer, x1 as i32, y1 as i32, x2 as i32, y2 as i32);
        }
    }
    pub fn circle(&self, root_x: isize, root_y: isize, r: f32, color: usize, alpha: u8) {
        let mut points = Vec::new();

        let step = (1f32 - 1f32 / (r as f32)).acos();

        let mut angle = 0f32;
        while angle < 360f32 {
            let rad = angle as f32 * PI / 180f32;
            let x = (r as f32 * rad.cos()) as i32 + root_x as i32;
            let y = (r as f32 * rad.sin()) as i32 + root_y as i32;
            points.push(SDL_Point { x, y });
            angle += step;
        }

        self.set_color(color, alpha);
        unsafe {
            SDL_RenderDrawPoints(self.renderer, points.as_ptr(), points.len() as i32);
        }
    }

    pub fn set_color(&self, color: usize, a: u8) {
        let (r, g, b) = hex_to_rgb!(color);
        unsafe {
            SDL_SetRenderDrawBlendMode(self.renderer, SDL_BlendMode::SDL_BLENDMODE_BLEND);
            SDL_SetRenderDrawColor(self.renderer, r, g, b, a);
        }
    }
    pub fn text(
        &mut self,
        text: &str,
        mut x: isize,
        mut y: isize,
        size: FontSize,
        center_horizontaly: bool,
        color: usize,
        alpha: u8,
    ) {
        if text.len() == 0 {
            return;
        }

        let text_size = d!().fonts.get_text_size(text, size.clone());
        y += (text_size.1 + text_size.2) / 2;
        if center_horizontaly {
            x -= text_size.0 / 2;
        }
        let face = self.fonts.get_face(&size);

        for letter in text.chars() {
            x -= 1;
            let advance = d!()
                .fonts
                .draw_glyph(&face, letter, size.clone(), x, y, color, alpha);

            x += advance.0;
            y += advance.1;
        }
    }
}
