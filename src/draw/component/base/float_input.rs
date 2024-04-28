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

#[derive(Debug)]
pub struct FloatInput {
    text_input: TextInput,
    text_val: Arcm<String>,
    float_val: Arcm<f32>,
    validator: fn(f32) -> bool,
}

impl FloatInput {
    pub fn new(
        x: isize,
        y: isize,
        label: &'static str,
        val: Arcm<f32>,
        validator: Option<fn(f32) -> bool>,
    ) -> FloatInput {
        let text_val = Arcm::new(val.lock().unwrap().to_string());
        let validator = validator.unwrap_or(|_| true);

        FloatInput {
            text_input: TextInput::new(x, y, label, text_val.clone()),
            text_val,
            float_val: val,
            validator,
        }
    }
}

impl Component for FloatInput {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let mut float_val = self.float_val.lock().unwrap();
        let text_val = self.text_val.lock().unwrap();
        if let Ok(val) = text_val.parse() {
            if *float_val != val && (self.validator)(val) {
                *float_val = val;
            }
        }
        drop(text_val);
        self.text_input.draw(frame)?;
        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.text_input.handle_event(event)
    }

    fn get_base(&mut self) -> &mut ComponentBase {
        self.text_input.get_base()
    }
}
