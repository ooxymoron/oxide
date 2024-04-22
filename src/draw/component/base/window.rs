use crate::{
    d, draw::{
        colors::{BACKGROUND, CURSOR, FOREGROUND}, component::{Component, Components, DrawOrder}, event::{Event, EventType}, fonts::FontSize, frame::Frame
    }, error::OxideResult, util::{arcm::Arcm, point_in_bounds}
};

use super::button::Button;

const HEADER_HEIGHT: isize = 50;

#[derive(Debug)]
pub struct Window {
    x: isize,
    y: isize,
    w: isize,
    h: isize,
    rooted_x: isize,
    rooted_y: isize,
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
    pub fn new(title: String, visible: Arcm<bool>, components: Components) -> Window {
        let w = 800;
        let h = 800;

        let close_button_size = FontSize::Small as isize + 2;
        let close_button_pad = HEADER_HEIGHT / 2 - close_button_size / 2;
        let close_button = Button::new(
            "x",
            w - close_button_pad - close_button_size,
            close_button_pad,
            close_button_size,
            close_button_size,
            visible.clone(),
            FontSize::Small,
        );
        Window {
            x: 100,
            y: 100,
            rooted_x: 0,
            rooted_y: 0,
            w,
            h,
            title,
            last_cursor: (0, 0),
            visible,
            last_visible: false,
            draw_order: DrawOrder::Value(0),
            dragging: false,
            components,
            close_button,
        }
    }
}

impl Component for Window {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) -> OxideResult<()>{
        let x = root_x + self.x;
        let y = root_y + self.y;
        self.rooted_x = x;
        self.rooted_y = y;
        if !*self.visible.lock().unwrap() {
            self.last_visible = false;
            return Ok(());
        }
        if !self.last_visible {
            self.draw_order = DrawOrder::Top
        }
        frame.filled_rect(x, y, self.w, HEADER_HEIGHT, BACKGROUND, 255);
        frame.filled_rect(
            x,
            y + HEADER_HEIGHT,
            self.w,
            self.h - HEADER_HEIGHT,
            BACKGROUND,
            220,
        );

        frame.text(
            &self.title,
            x + self.w / 2,
            y + HEADER_HEIGHT / 2,
            FontSize::Medium,
            true,
            FOREGROUND,
            255,
        );

        frame.filled_rect(x, y + HEADER_HEIGHT, self.w, 1, CURSOR, 100);
        frame.outlined_rect(x, y, self.w, self.h, CURSOR, 255);

        self.components.draw(frame, x, y + HEADER_HEIGHT)?;
        self.close_button.draw(frame, x, y)?;
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
        match event.r#type {
            EventType::CursorMove(pos) => {
                if self.dragging {
                    self.x += pos.0 as isize - self.last_cursor.0;
                    self.y += pos.1 as isize - self.last_cursor.1;
                }
            }
            EventType::MouseButtonDown => {
                if point_in_bounds(
                    d!().cursor.0,
                    d!().cursor.1,
                    self.rooted_x,
                    self.rooted_y,
                    self.w,
                    HEADER_HEIGHT,
                ) {
                    self.dragging = true;
                }
                if point_in_bounds(
                    d!().cursor.0,
                    d!().cursor.1,
                    self.rooted_x,
                    self.rooted_y,
                    self.w,
                    self.h,
                ) {
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
}


