use std::borrow::BorrowMut;

use crate::{
    d,
    draw::{
        colors::FOREGROUND,
        component::{Component, ComponentBase},
        fonts::FontSize,
    },
    util::arcm::Arcm,
};

const PAD: isize = 6;

#[derive(Debug)]
pub struct Select {
    base: ComponentBase,
    options: Vec<String>,
    selected: Arcm<Vec<String>>,
}

impl Select {
    pub fn new(options: Vec<String>, selected: Arcm<Vec<String>>) -> Select {
        let mut w = 0;
        for option in &options {
            let size = d!().fonts.get_text_size(option, FontSize::Medium);
            w = w.max(size.0 + PAD * 2)
        }

        Select {
            base: ComponentBase {
                x: 0,
                y: 0,
                w,
                h: FontSize::Medium.height() + 2 * PAD,
            },
            options,
            selected,
        }
    }
}

impl Component for Select {
    fn get_base(&mut self) -> &mut ComponentBase {
        self.base.borrow_mut()
    }
    fn draw(&mut self, frame: &mut crate::draw::frame::Frame) -> crate::error::OxideResult<()> {
        let ComponentBase { x, y, w, h } = self.base;
        frame.outlined_rect(x, y, w, h, FOREGROUND, 255);
        let selected = self.selected.lock().unwrap();
        let selected_text = selected.first().unwrap();
        frame.text(
            selected_text,
            x,
            y,
            FontSize::Medium,
            false,
            false,
            FOREGROUND,
            255,
        );
        Ok(())
    }
}
