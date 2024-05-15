use std::borrow::BorrowMut;

use crate::{
    d,
    draw::{
        colors::{FOREGROUND3, BACKGROUND2, FOREGROUND},
        component::{Component, ComponentBase},
        event::{Event, EventType},
        fonts::FontSize,
        frame::Frame,
    },
    error::OxideResult,
    util::{arcm::Arcm, point_in_bounds},
};

#[derive(Debug)]
pub struct Button {
    base: ComponentBase,
    val: Arcm<bool>,
    text: String,
    size: FontSize,
}

impl Button {
    pub fn new(base: ComponentBase, text: &str, val: Arcm<bool>, size: FontSize) -> Button {
        Button {
            base,
            val,
            text: text.to_owned(),
            size,
        }
    }
}

impl Component for Button {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let ComponentBase { x, y, w, h } = self.base;
        frame.filled_rect(x, y, w, h, BACKGROUND2, 255);
        frame.outlined_rect(x, y, w, h, FOREGROUND3, 255);
        frame.text(
            &self.text,
            x + w / 2,
            y + h / 2,
            self.size.clone(),
            true,
            true,
            FOREGROUND,
            255,
        );
        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown(1) => {
                let ComponentBase { x, y, w, h } = self.base;
                if point_in_bounds(d!().cursor.0, d!().cursor.1, x, y, w, h) {
                    let mut val = self.val.lock().unwrap();
                    *val = !*val;
                    event.handled = true;
                }
            }
            _ => (),
        }
    }

    fn get_base(&mut self) -> &mut ComponentBase {
        self.base.borrow_mut()
    }
}
