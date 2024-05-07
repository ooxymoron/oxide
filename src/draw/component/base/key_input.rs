use std::borrow::BorrowMut;

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
    util::{arcm::Arcm, point_in_bounds, scancode::Scancode, sdl_scancode_name_to_string},
};

const SIZE: isize = FontSize::Small as isize + 4;

#[derive(Debug)]
pub struct KeyInput {
    base: ComponentBase,
    label: &'static str,
    val: Arcm<Scancode>,
    focussed: bool,
    input_w: isize
}

impl KeyInput {
    pub fn new(x: isize, y: isize, label: &'static str, val: Arcm<Scancode>) -> KeyInput {
        let text_size = d!().fonts.get_text_size(label, FontSize::Small);
        let input_w = 100;
        let w = text_size.0 + input_w + 10;
        let h = SIZE;
        KeyInput {
            base: ComponentBase { x, y, w, h },
            label,
            val,
            focussed: false,
            input_w
        }
    }
}

impl Component for KeyInput {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let ComponentBase { x, y, w:_, h } = self.base;
        frame.filled_rect(x, y, self.input_w, h, BACKGROUND, 255);

        let outline = if self.focussed { BLUE } else { FOREGROUND };
        frame.outlined_rect(x, y, self.input_w, h, outline, 255);

        let val = *self.val.lock().unwrap();

        frame.text(
            &sdl_scancode_name_to_string(*val),
            x + self.input_w / 2,
            y + h / 2,
            FontSize::Small,
            true,
            true,
            FOREGROUND,
            255,
        );
        frame.text(
            &self.label,
            x + self.input_w + 10,
            y + h / 2,
            FontSize::Small,
            false,
            true,
            FOREGROUND,
            255,
        );
        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown => {
        let ComponentBase{x,y,w,h} = self.base;
                if !self.focussed {
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
                *self.val.lock().unwrap() = Scancode::new(key);
                event.handled = true;
                self.focussed = false;
            }
            _ => (),
        }
    }

    fn get_base(&mut self) -> &mut ComponentBase {
        self.base.borrow_mut()
    }
}
