use crate::{
    draw::{
        component::{Component, ComponentBase},
        event::Event,
        frame::Frame,
    },
    error::OxideResult,
};

use super::window::Window;

#[derive(Debug)]
pub struct VisibleWindow {
    pub window: Window,
}
impl VisibleWindow {
    pub fn new(window: Window) -> VisibleWindow {
        VisibleWindow { window }
    }
    pub fn draw_hidden(&mut self, frame: &mut Frame) -> OxideResult<()> {
        self.window.components.draw(frame)?;
        Ok(())
    }
}
impl Component for VisibleWindow {
    fn handle_event(&mut self, event: &mut Event) {
        self.window.handle_event(event);
    }
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        self.window.draw(frame)?;
        Ok(())
    }
    fn get_base(&mut self) -> &mut ComponentBase {
        self.window.get_base()
    }
}
