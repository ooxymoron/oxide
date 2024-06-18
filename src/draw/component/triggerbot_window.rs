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
    base::{checkbox::Checkbox, window::Window},
    Component, ComponentBase,
};

#[derive(Debug)]
pub struct TriggerbotWindow {
    window: Window,
}

impl TriggerbotWindow {
    pub fn new(visible: Arcm<bool>) -> TriggerbotWindow {
        let mut window = Window::new("TRIGGERBOT".to_owned(), Some(visible));

        let mut container = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 16, 10);

        container.add(Checkbox::new(
            "sticky",
            s!().triggerbot.sticky.clone(),
            0,
            0,
        ));
        container.add(Checkbox::new(
            "avoid self damage",
            s!().triggerbot.avoid_self_damage.clone(),
            0,
            0,
        ));

        window.add(container);

        TriggerbotWindow { window }
    }
}

impl Component for TriggerbotWindow {
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
