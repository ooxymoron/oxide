use std::borrow::BorrowMut;

use crate::{
    d,
    draw::{
        colors::FOREGROUND,
        component::{Component, ComponentBase},
        fonts::FontSize,
        frame::Frame,
    },
    error::OxideResult,
};

const PADDING: isize = 2;

#[derive(Debug)]
pub struct Label {
    text: String,
    base: ComponentBase,
    color: usize
}

impl Label {
    pub fn new(text: String, x: isize, y: isize, color: usize) -> Label {
        let size = d!().fonts.get_text_size(&text, FontSize::Small);
        let w = size.0 + PADDING * 2;
        let h = size.1 + size.2 + PADDING * 2;
        Label {
            text,
            base: ComponentBase { x, y, w, h },
            color
        }
    }
}

impl Component for Label {
    fn get_base(&mut self) -> &mut ComponentBase {
        self.base.borrow_mut()
    }
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let base = &self.base;
        frame.text(
            &self.text,
            base.x + PADDING,
            base.y + self.base.h - PADDING,
            FontSize::Small,
            false,
            false,
            self.color,
            255,
        );
        Ok(())
    }
}
