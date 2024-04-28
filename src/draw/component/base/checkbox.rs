use std::borrow::BorrowMut;

use crate::{
    d,
    draw::{
        colors::{BACKGROUND, FOREGROUND},
        component::{Component, ComponentBase},
        event::{Event, EventType},
        fonts::FontSize,
        frame::Frame,
    },
    error::OxideResult,
    util::arcm::Arcm,
};

const SIZE: isize = 12;

#[derive(Debug, Clone)]
pub struct Checkbox {
    base: ComponentBase,
    pub checked: Arcm<bool>,
    label: &'static str,
}
impl Checkbox {
    pub fn new(label: &'static str, checked: Arcm<bool>, x: isize, y: isize) -> Checkbox {
        let text_size = d!().fonts.get_text_size(label, FontSize::Small);
        let w = text_size.0 + SIZE + 10;
        let h = SIZE;
        Checkbox {
            base: ComponentBase { x, y, w, h },
            checked,
            label,
        }
    }
}
impl Component for Checkbox {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let ComponentBase { x, y, w:_, h:_ } = self.base;
        frame.filled_rect(x, y, SIZE, SIZE, FOREGROUND, 255);
        if !*self.checked.lock().unwrap() {
            frame.filled_rect(x + 1, y + 1, SIZE - 2, SIZE - 2, BACKGROUND, 255);
        }
        frame.text(
            self.label,
            x + SIZE + 10,
            y + SIZE / 2,
            FontSize::Small,
            false,
            FOREGROUND,
            255,
        );
        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown => {
                let ComponentBase { x, y, w:_, h:_ } = self.base;
                if d!().cursor.0 as isize <= x + SIZE
                    && x <= d!().cursor.0 as isize
                    && d!().cursor.1 as isize <= y + SIZE
                    && y <= d!().cursor.1 as isize
                {
                    let mut checked = self.checked.lock().unwrap();
                    *checked = !*checked;
                    event.handled = true;
                }
            }
            _ => (),
        }
    }

    fn get_base(&mut self) -> &mut crate::draw::component::ComponentBase {
        self.base.borrow_mut()
    }
}
