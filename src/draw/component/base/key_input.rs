use crate::{
    d, draw::{
        colors::{BACKGROUND, BLUE, FOREGROUND},
        component::Component,
        event::{Event, EventType},
        fonts::FontSize,
        frame::Frame,
    }, error::OxideResult, util::{arcm::Arcm, point_in_bounds, scancode::Scancode, sdl_scancode_name_to_string}
};

const SIZE: isize = FontSize::Small as isize + 4;

#[derive(Debug)]
pub struct KeyInput {
    label: &'static str,
    x: isize,
    y: isize,
    w: isize,
    rooted_x: isize,
    rooted_y: isize,
    val: Arcm<Scancode>,
    focussed: bool,
}

impl KeyInput {
    pub fn new(label: &'static str, x: isize, y: isize, val: Arcm<Scancode>) -> KeyInput {
        KeyInput {
            label,
            x,
            y,
            w: 100,
            rooted_x: 0,
            rooted_y: 0,
            val,
            focussed: false,
        }
    }
}

impl Component for KeyInput {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) -> OxideResult<()>{
        let x = self.x + root_x;
        let y = self.y + root_y;

        self.rooted_x = x;
        self.rooted_y = y;

        frame.filled_rect(x, y, self.w, SIZE, BACKGROUND, 255);

        let outline = if self.focussed { BLUE } else { FOREGROUND };
        frame.outlined_rect(x, y, self.w, SIZE, outline, 255);

        let val = *self.val.lock().unwrap();

        frame.text(
            &sdl_scancode_name_to_string(*val),
            x + self.w / 2,
            y + SIZE / 2,
            FontSize::Small,
            true,
            FOREGROUND,
            255,
        );
        frame.text(
            &self.label,
            x + self.w + 10,
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
                if !self.focussed {
                    if point_in_bounds(
                        d!().cursor.0,
                        d!().cursor.1,
                        self.rooted_x,
                        self.rooted_y,
                        self.w,
                        SIZE,
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
    fn height(&self) -> isize {
        SIZE
    }
}
