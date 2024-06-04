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

#[derive(Debug)]
pub struct TextInput {
    base: ComponentBase,
    label: &'static str,
    input_w: isize,
    val: Arcm<String>,
    focussed: bool,
}

impl TextInput {
    pub fn new( x: isize, y: isize, label: &'static str, val: Arcm<String>) -> TextInput {
        let text_size = d!().fonts.get_text_size(label, FontSize::Medium);
        let input_w = 100;
        let w = text_size.0 + input_w + 10;
        let h = SIZE;
        TextInput {
            base: ComponentBase{x,y,w,h},
            label,
            input_w,
            val,
            focussed: false,
        }
    }
}

impl Component for TextInput {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let ComponentBase{x,y,w:_,h} = self.base;

        frame.filled_rect(x, y, self.input_w, SIZE, BACKGROUND, 255);
        let outline = if self.focussed { BLUE } else { FOREGROUND };
        frame.outlined_rect(x, y, self.input_w, SIZE, outline, 255);

        frame.text(
            &*self.val.lock().unwrap(),
            x + self.input_w / 2,
            y + h / 2,
            FontSize::Medium,
            true,
            true,
            FOREGROUND,
            255,
        );

        frame.text(
            &self.label,
            x + self.input_w + 10,
            y + h / 2,
            FontSize::Medium,
            false,
            true,
            FOREGROUND,
            255,
        );

        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown(1) => {
                if !self.focussed {
                    let ComponentBase{x,y,w,h} = self.base;
                    if point_in_bounds(
                        d!().cursor.0,
                        d!().cursor.1,
                        x,
                        y,
                        w,
                        h,
                    ) {
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
