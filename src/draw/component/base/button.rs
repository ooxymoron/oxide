use crate::{
    d, draw::{
        colors::{CURSOR, CURSOR_TEXT, FOREGROUND},
        component::Component,
        event::{Event, EventType},
        fonts::FontSize,
        frame::Frame,
    }, error::OxideResult, util::{arcm::Arcm, point_in_bounds} 
};

#[derive(Debug)]
pub struct Button {
    x: isize,
    y: isize,
    rooted_x: isize,
    rooted_y: isize,
    w: isize,
    h: isize,
    val: Arcm<bool>,
    text: String,
    size: FontSize,
}

impl Button {
    pub fn new(
        text: &str,
        x: isize,
        y: isize,
        w: isize,
        h: isize,
        val: Arcm<bool>,
        size: FontSize,
    ) -> Button {
        Button {
            x,
            y,
            rooted_x: 0,
            rooted_y: 0,
            w,
            h,
            val,
            text: text.to_owned(),
            size,
        }
    }
}

impl Component for Button {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) -> OxideResult<()>{
        let x = self.x + root_x;
        let y = self.y + root_y;
        self.rooted_x = x;
        self.rooted_y = y;
        frame.filled_rect(x, y, self.w, self.h, CURSOR_TEXT, 255);
        frame.outlined_rect(x, y, self.w, self.h, CURSOR, 255);
        frame.text(
            &self.text,
            x + self.w / 2 - 1,
            y + self.h / 2 + 1,
            self.size.clone(),
            true,
            FOREGROUND,
            255,
        );
        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown => {
                if point_in_bounds(
                    d!().cursor.0,
                    d!().cursor.1,
                    self.rooted_x,
                    self.rooted_y,
                    self.w,
                    self.h,
                ) {
                    let mut val = self.val.lock().unwrap();
                    *val = !*val;
                    event.handled = true;
                }
            }
            _ => (),
        }
    }
}

