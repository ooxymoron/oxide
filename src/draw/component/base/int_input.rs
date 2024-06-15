use crate::{
    draw::{
        component::{Component, ComponentBase},
        event::Event,
        frame::Frame,
    },
    error::OxideResult,
    util::arcm::Arcm,
};

use super::text_input::TextInput;

#[derive(Debug, Clone)]
pub struct IntInput {
    pub text_input: TextInput,
    text_val: Arcm<String>,
    int_val: Arcm<isize>,
    validator: fn(isize) -> bool,
}

impl IntInput {
    pub fn new(
        x: isize,
        y: isize,
        label: Option<String>,
        val: Arcm<isize>,
        validator: Option<fn(isize) -> bool>,
    ) -> IntInput {
        let text_val = Arcm::new(val.lock().unwrap().to_string());
        let validator = validator.unwrap_or(|_| true);

        IntInput {
            text_input: TextInput::new(x, y, label, text_val.clone()),
            text_val,
            int_val: val,
            validator,
        }
    }
}

impl Component for IntInput {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        if self.text_input.focussed {
            let mut float_val = self.int_val.lock().unwrap();
            let text_val = self.text_val.lock().unwrap();
            if let Ok(val) = text_val.parse() {
                if *float_val != val && (self.validator)(val) {
                    *float_val = val;
                }
            }
        }
        self.text_input.draw(frame)?;
        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.text_input.get_base().x = self.get_base().x;
        self.text_input.get_base().y = self.get_base().y;
        self.text_input.handle_event(event)
    }

    fn get_base(&mut self) -> &mut ComponentBase {
        self.text_input.get_base()
    }
}
