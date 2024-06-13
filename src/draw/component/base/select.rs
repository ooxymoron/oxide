use std::borrow::BorrowMut;

use crate::{
    d,
    draw::{
        colors::{BACKGROUND, FOREGROUND},
        component::{Component, ComponentBase, DrawOrder},
        event::{Event, EventType},
        fonts::FontSize,
    },
    util::{arcm::Arcm, point_in_bounds},
};

const PAD: isize = 6;

#[derive(Debug, Clone)]
pub struct Select {
    base: ComponentBase,
    #[allow(unused)]
    options: Vec<String>,
    selected: Arcm<(bool,Vec<String>)>,
    multiple: bool,
    backgournd: bool,
    dropdown: bool,
}

impl Select {
    pub fn new(options: Vec<String>, selected: Arcm<(bool,Vec<String>)>, multiple: bool) -> Select {
        let mut w = PAD * 4;
        if multiple {
            let selected = selected.lock().unwrap();
            let selected_text = selected.1.join(", ");
            w = d!().fonts.get_text_size(&selected_text, FontSize::Medium).0 + 4 * PAD;
        }
        for option in &options {
            let size = d!().fonts.get_text_size(option, FontSize::Medium);
            w = w.max(size.0 + PAD * 4)
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
            multiple,
            backgournd: true,
            dropdown: false,
        }
    }
    fn draw_dropdown(&mut self, frame: &mut crate::draw::frame::Frame) {
        let selected = self.selected.lock().unwrap();
        let ComponentBase { x, mut y, w, h } = self.base;
        y += h;
        let background_h = self.options.len() as isize * (FontSize::Medium.height() + PAD) + PAD;
        frame.filled_rect(x, y, w, background_h, BACKGROUND, 250);
        frame.outlined_rect(x, y, w, background_h, FOREGROUND, 255);
        for option in &self.options {
            y += FontSize::Medium.height() + PAD;
            frame.text(
                option,
                x + PAD,
                y,
                FontSize::Medium,
                false,
                false,
                FOREGROUND,
                255,
            );
            frame.line(x, y + PAD, x + w - 1, y + PAD, FOREGROUND, 255);
            if selected.1.contains(option) {
                frame.text(
                    "",
                    x + w - 3 * PAD,
                    y,
                    FontSize::Medium,
                    false,
                    false,
                    FOREGROUND,
                    255,
                )
            }
        }
    }
    fn update_size(&mut self) {
        self.get_base().w = PAD * 4;
        for option in self.options.clone() {
            let size = d!().fonts.get_text_size(&option, FontSize::Medium);
            self.get_base().w = self.get_base().w.max(size.0 + PAD * 4)
        }
        if self.multiple {
            let selected = self.selected.clone();
            let selected = selected.lock().unwrap();
            let selected_text = selected.1.join(", ");
            self.get_base().w = self
                .get_base()
                .w
                .max(d!().fonts.get_text_size(&selected_text, FontSize::Medium).0 + 4 * PAD);
        }
    }

}

impl Component for Select {
    fn get_base(&mut self) -> &mut ComponentBase {
        self.base.borrow_mut()
    }
    fn draw(&mut self, frame: &mut crate::draw::frame::Frame) -> crate::error::OxideResult<()> {
        let ComponentBase { x, y, w, h } = self.base;
        if self.backgournd {
            frame.outlined_rect(x, y, w, h, FOREGROUND, 255);
        }
        let selected = self.selected.lock().unwrap();
        let selected_text = selected.1.join(", ");
        let y = y + (h + FontSize::Medium.height()) / 2;
        frame.text(
            &selected_text,
            x + PAD,
            y,
            FontSize::Medium,
            false,
            false,
            FOREGROUND,
            255,
        );
        frame.text(
            "󱞣",
            x + w - 3 * PAD,
            y,
            FontSize::Medium,
            false,
            false,
            FOREGROUND,
            255,
        );
        drop(selected);
        if self.dropdown {
            self.draw_dropdown(frame)
        }
        self.update_size();
        Ok(())
    }
    fn handle_event(&mut self, event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown(1) => {
                if point_in_bounds(d!().cursor.0, d!().cursor.1, &self.base) {
                    self.dropdown = !self.dropdown;
                    event.handled = true;
                    return
                }
                if self.dropdown {
                    let mut base = self.base.clone();
                    base.y += base.h + PAD;
                    let size = FontSize::Medium.height() + PAD;
                    base.h = size;
                    for option in &self.options {
                        if point_in_bounds(d!().cursor.0, d!().cursor.1, &base) {
                            let mut selected = self.selected.lock().unwrap();
                            selected.0 = true;
                            if selected.1.contains(option) {
                                selected.1.retain(|x| x != option);
                            } else {
                                selected.1.push(option.clone());
                            }
                            event.handled = true;
                            return
                        }
                        base.y += size;
                    }
                }
            }
            _ => {}
        }
    }
    fn get_draw_order(&self) -> crate::draw::component::DrawOrder {
        if self.dropdown {
            return DrawOrder::Top;
        }
        DrawOrder::Value(0)
        
    }
}
