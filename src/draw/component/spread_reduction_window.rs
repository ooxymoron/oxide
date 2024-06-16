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
pub struct SpreadReductionWindow {
    window: Window,
}

impl SpreadReductionWindow {
    pub fn new(visible: Arcm<bool>) -> SpreadReductionWindow {
        let mut window = Window::new("SPREAD REDUCTION".to_owned(), Some(visible));

        let mut container = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 16, 10);

        container.add(Checkbox::new(
            "spread reduction",
            s!().spread_reduction.seed_prediction.clone(),
            0,
            0,
        ));
        container.add(Checkbox::new(
            "tapfire",
            s!().spread_reduction.tapfire.clone(),
            0,
            0,
        ));
        container.add(Checkbox::new(
            "tapfire on manual shots",
            s!().spread_reduction.tapfire_on_manual_shots.clone(),
            0,
            0,
        ));
        container.add(Checkbox::new(
            "tapfire only minigun",
            s!().spread_reduction.tapfire_only_minigun.clone(),
            0,
            0,
        ));
        window.add(container);
        SpreadReductionWindow { window }
    }
}

impl Component for SpreadReductionWindow {
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
