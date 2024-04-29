use std::{borrow::BorrowMut, isize};

use crate::{
    d,
    draw::{
        colors::{BACKGROUND, CURSOR, FOREGROUND},
        component::{Component, ComponentBase, Components, DrawOrder},
        event::{Event, EventType},
        fonts::FontSize,
        frame::Frame,
    },
    error::OxideResult,
    util::{arcm::Arcm, point_in_bounds},
};

use super::button::Button;

const HEADER_HEIGHT: isize = 50;
const CLOSE_BUTTON_SIZE: isize = FontSize::Small as isize + 2;
const PADDING: isize = HEADER_HEIGHT / 2 - CLOSE_BUTTON_SIZE / 2;

#[derive(Debug)]
pub struct Window {
    base: ComponentBase,
    title: String,
    last_cursor: (isize, isize),
    pub visible: Arcm<bool>,
    last_visible: bool,
    draw_order: DrawOrder,
    dragging: bool,
    components: Components,
    close_button: Button,
}

impl Window {
    pub fn new(title: String, visible: Arcm<bool>) -> Window {
        let x = 100;
        let y = 100;
        let w =
            d!().fonts.get_text_size(&title, FontSize::Medium).0 + CLOSE_BUTTON_SIZE + PADDING * 3;
        let close_button = Button::new(
            ComponentBase {
                x: x + w - CLOSE_BUTTON_SIZE - PADDING,
                y: y + PADDING,
                w: CLOSE_BUTTON_SIZE,
                h: CLOSE_BUTTON_SIZE,
            },
            "x",
            visible.clone(),
            FontSize::Small,
        );

        Window {
            base: ComponentBase {
                x,
                y,
                w,
                h: HEADER_HEIGHT,
            },
            title,
            last_cursor: (0, 0),
            visible,
            last_visible: false,
            draw_order: DrawOrder::Value(0),
            dragging: false,
            components: Components::new(),
            close_button,
        }
    }
    pub fn add(&mut self, mut component: impl Component + 'static) {
        let component_base = component.get_base();
        self.base.h = self.base.h.max(component_base.y + component_base.h + HEADER_HEIGHT + PADDING);
        self.base.w = self.base.w.max(component_base.x + component_base.w + PADDING);
        let button_base = self.close_button.get_base();
        button_base.x = self.base.x + self.base.w - CLOSE_BUTTON_SIZE - PADDING;

        component_base.x += self.base.x;
        component_base.y += self.base.y + HEADER_HEIGHT;

        self.components.add(component)
    }
}

impl Component for Window {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let ComponentBase { x, y, w, h } = self.base;
        if !*self.visible.lock().unwrap() {
            self.last_visible = false;
            return Ok(());
        }
        if !self.last_visible {
            self.draw_order = DrawOrder::Top
        }
        frame.filled_rect(x, y, w, HEADER_HEIGHT, BACKGROUND, 255);
        frame.filled_rect(x, y + HEADER_HEIGHT, w, h - HEADER_HEIGHT, BACKGROUND, 220);

        frame.text(
            &self.title,
            x + (w - PADDING - CLOSE_BUTTON_SIZE) / 2,
            y + HEADER_HEIGHT / 2,
            FontSize::Medium,
            true,
            FOREGROUND,
            255,
        );

        frame.filled_rect(x, y + HEADER_HEIGHT, w, 1, CURSOR, 100);
        frame.outlined_rect(x, y, w, h, CURSOR, 255);

        self.components.draw(frame)?;
        self.close_button.draw(frame)?;
        self.last_visible = true;
        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        if !*self.visible.lock().unwrap() {
            return;
        }
        self.components.handle_event(event);
        if event.handled {
            return;
        }
        self.close_button.handle_event(event);
        if event.handled {
            return;
        }
        let ComponentBase { x, y, w, h } = self.base.borrow_mut();
        match event.r#type {
            EventType::CursorMove(pos) => {
                if self.dragging {
                    let diff = (
                        pos.0 as isize - self.last_cursor.0,
                        pos.1 as isize - self.last_cursor.1,
                    );
                    *x += diff.0;
                    *y += diff.1;
                    self.components.0.iter_mut().for_each(|component| {
                        let ComponentBase { x, y, w: _, h: _ } = component.get_base();
                        *x += diff.0;
                        *y += diff.1;
                    });
                    let ComponentBase { x, y, w: _, h: _ } = self.close_button.get_base();
                    *x += diff.0;
                    *y += diff.1;
                }
            }
            EventType::MouseButtonDown => {
                if point_in_bounds(d!().cursor.0, d!().cursor.1, *x, *y, *w, HEADER_HEIGHT) {
                    self.dragging = true;
                }
                if point_in_bounds(d!().cursor.0, d!().cursor.1, *x, *y, *w, *h) {
                    self.draw_order = DrawOrder::Top;
                    event.handled = true;
                }
            }
            EventType::MouseButtonUp => {
                self.dragging = false;
            }
            _ => (),
        }
        self.last_cursor = d!().cursor;
    }
    fn get_draw_order(&self) -> DrawOrder {
        self.draw_order.clone()
    }
    fn set_draw_order(&mut self, order: DrawOrder) {
        self.draw_order = order
    }
    fn get_base(&mut self) -> &mut ComponentBase {
        self.base.borrow_mut()
    }
}
