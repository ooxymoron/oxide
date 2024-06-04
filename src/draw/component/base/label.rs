use std::{borrow::BorrowMut, ffi::CString, mem::transmute};

use sdl2_sys::SDL_SetClipboardText;

use crate::{
    d,
    draw::{
        component::{Component, ComponentBase},
        event::{Event, EventType},
        fonts::FontSize,
        frame::Frame,
    },
    error::OxideResult,
    log,
    util::point_in_bounds,
};

const PADDING: isize = 6;

#[derive(Debug)]
pub struct Label {
    text: String,
    base: ComponentBase,
    color: usize,
    pub copy: bool,
}

impl Label {
    pub fn new(text: String, x: isize, y: isize, color: usize) -> Label {
        let size = d!().fonts.get_text_size(&text, FontSize::Medium);
        let w = size.0 + PADDING * 2;
        let h = FontSize::Medium.height() + PADDING * 2;
        Label {
            text,
            base: ComponentBase { x, y, w, h },
            color,
            copy: false,
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
            FontSize::Medium,
            false,
            false,
            self.color,
            255,
        );
        Ok(())
    }
    fn handle_event(&mut self, event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown(1) => {
                if point_in_bounds(d!().cursor.0, d!().cursor.1, &self.base) && self.copy {
                    unsafe {
                        log!("coppied {}", &self.text);
                        let text = CString::new(self.text.clone()).unwrap();
                        SDL_SetClipboardText(text.as_ptr());
                    }
                    event.handled = true;
                }
            }
            _ => {}
        }
    }
}
