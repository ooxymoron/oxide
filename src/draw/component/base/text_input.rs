use std::borrow::BorrowMut;

use sdl2_sys::*;

use crate::{
    d,
    draw::{
        colors::{BACKGROUND, BLUE, FOREGROUND},
        component::{Component, ComponentBase},
        event::{Event, EventType},
        fonts::FontSize,
        frame::Frame,
    },
    error::OxideResult,
    util::{arcm::Arcm, point_in_bounds, sdl_scancode_to_char},
};

const SIZE: isize = FontSize::Medium as isize + 4;

#[derive(Debug, Clone)]
pub struct TextInput {
    base: ComponentBase,
    label: Option<String>,
    input_w: isize,
    val: Arcm<String>,
    focussed: bool,
    pub background: bool
}

impl TextInput {
    pub fn new(x: isize, y: isize, label: Option<String>, val: Arcm<String>) -> TextInput {
        let text_size = if let Some(label) = &label {
            d!().fonts.get_text_size(label, FontSize::Medium)
        } else {
            (0, 0, 0)
        };

        let input_w = 100;
        let w = text_size.0 + input_w;
        let h = FontSize::Medium.height();
        TextInput {
            base: ComponentBase { x, y, w, h },
            label,
            input_w,
            val,
            focussed: false,
            background: true,
        }
    }
}

impl Component for TextInput {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let ComponentBase { x, y, w: _, h } = self.base;

        let mut text_color = FOREGROUND;
        if self.background {
            frame.filled_rect(x, y, self.input_w, SIZE, BACKGROUND, 255);
            let outline = if self.focussed { BLUE } else { FOREGROUND };
            frame.outlined_rect(x, y, self.input_w, SIZE, outline, 255);
        } else {
            if self.focussed {
                text_color = BLUE;
            }
        }

        frame.text(
            &*self.val.lock().unwrap(),
            x + self.input_w / 2,
            y + h / 2,
            FontSize::Medium,
            true,
            true,
            text_color,
            255,
        );

        if let Some(label) = &self.label {
            frame.text(
                label,
                x + self.input_w + 10,
                y + h / 2,
                FontSize::Medium,
                false,
                true,
                FOREGROUND,
                255,
            );
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown(1) => {
                if !self.focussed {
                    if point_in_bounds(d!().cursor.0, d!().cursor.1, &self.base) {
                        self.focussed = true;
                        event.handled = true;
                    }
                } else {
                    self.focussed = false;
                    event.handled = true;
                }
            }
            EventType::KeyDown(key) => {
                if !self.focussed {
                    return;
                }
                if let Some(letter) = sdl_scancode_to_char(key) {
                    let mut val = self.val.lock().unwrap();
                    val.push(letter);
                }
                match key {
                    SDL_Scancode::SDL_SCANCODE_DELETE => {}
                    SDL_Scancode::SDL_SCANCODE_BACKSPACE => {
                        let mut val = self.val.lock().unwrap();
                        val.pop();
                    }
                    SDL_Scancode::SDL_SCANCODE_RETURN | SDL_Scancode::SDL_SCANCODE_ESCAPE => {
                        self.focussed = false;
                    }
                    _ => {}
                }
                event.handled = true
            }
            _ => (),
        }
    }
    fn get_base(&mut self) -> &mut ComponentBase {
        self.base.borrow_mut()
    }
}
