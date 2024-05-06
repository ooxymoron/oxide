use crate::{
    draw::{event::Event, frame::Frame},
    error::OxideResult,
    s,
    util::arcm::Arcm,
};

use super::{
    base::{checkbox::Checkbox, window::Window},
    Component, ComponentBase,
};

#[derive(Debug)]
pub struct MovementWindow {
    window: Window,
}

impl MovementWindow {
    pub fn new(visible: Arcm<bool>) -> MovementWindow {
        let mut window = Window::new("MOVEMENT".to_owned(), Some(visible));
        let mut y = 10;
        macro_rules! add {
            ($e:expr) => {
                #[allow(unused_assignments)]
                {
                    window.add($e, 8);
                    y += $e.get_base().h + 8;
                }
            };
        }

        add!(Checkbox::new("bhop", s!().movement.bhop.clone(), 10, y));
        add!(Checkbox::new(
            "autostrafe",
            s!().movement.autostrafe.clone(),
            10,
            y
        ));
        add!(Checkbox::new(
            "no push",
            s!().movement.no_push.clone(),
            10,
            y
        ));
        add!(Checkbox::new(
            "momentum compensation",
            s!().movement.momentum_compensation.clone(),
            10,
            y
        ));

        MovementWindow { window }
    }
}

impl Component for MovementWindow {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        self.window.draw(frame)
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
    fn get_base(&mut self) -> &mut ComponentBase {
        self.window.get_base()
    }
}
