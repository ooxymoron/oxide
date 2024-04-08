
use crate::{draw::{event::Event, frame::Frame}, error::OxideResult, s, util::arcm::Arcm};

use super::{base::{checkbox::Checkbox, window::Window}, Component, Components};


#[derive(Debug)]
pub struct MovementWindow {
    window: Window,
}

impl MovementWindow {
    pub fn new(visible: Arcm<bool>) -> MovementWindow {
        let mut components = Components::new();

        let mut y = 10;
        macro_rules! a {
            ($e:expr) => {
                components.add($e);
                #[allow(unused_assignments)]
                y += $e.height() + 8
            };
        }

        a!(Checkbox::new("bhop", s!().movement.bhop.clone(), 10, y));
        a!(Checkbox::new("autostrafe", s!().movement.autostrafe.clone(), 10, y));

        let window = Window::new("Movement".to_owned(), visible, components);
        MovementWindow { window }
    }
}

impl Component for MovementWindow {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) -> OxideResult<()>{
        self.window.draw(frame, root_x, root_y)
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.window.handle_event(event);
    }
    fn get_draw_order(&self) -> super::DrawOrder {
        self.window.get_draw_order()
    }
    fn set_draw_order(&mut self, order: super::DrawOrder) {
        self.window.set_draw_order(order)
    }
}
