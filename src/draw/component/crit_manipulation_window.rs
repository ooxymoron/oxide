use crate::{
    draw::{
        component::base::linear_layout::{LinearLayout, LinearLayoutOrientation},
        event::Event,
        frame::Frame,
    },
    error::OxideResult,
    s,
    util::arcm::Arcm,
};

use super::{
    base::{checkbox::Checkbox, key_input::KeyInput, window::Window},
    Component, ComponentBase,
};

#[derive(Debug)]
pub struct CritManipulationWindow {
    window: Window,
}

impl CritManipulationWindow {
    pub fn new(visible: Arcm<bool>) -> CritManipulationWindow {
        let mut window = Window::new("CRIT MANIPULATION".to_owned(), Some(visible));

        let mut container = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 16, 10);

        container.add(KeyInput::new(
            0,
            0,
            "crit key",
            s!().crit_manipulation.key.clone(),
        ));

        container.add(Checkbox::new(
            "auto cycle rapid fire",
            s!().crit_manipulation.auto_cycle_rapid_fire.clone(),
            0,
            0,
        ));

        window.add(container);

        CritManipulationWindow { window }
    }
}

impl Component for CritManipulationWindow {
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
