use crate::{
    draw::{colors::FOREGROUND, fonts::FontSize},
    error::OxideResult,
    o,
    oxide::cheat::visual::Visuals,
    sdk::entity::Entity,
    setting,
};

use super::Component;

#[derive(Debug)]
pub struct SpectatorList {}

impl SpectatorList {
    fn should_draw(&self) -> bool {
        if !setting!(visual, spectator_list) {
            return false;
        }
        if Entity::get_local().is_err() {
            return false;
        };
        return true;
    }
}

impl Component for SpectatorList {
    fn draw(
        &mut self,
        frame: &mut crate::draw::frame::Frame,
        _: isize,
        _: isize,
    ) -> OxideResult<()> {
        if !self.should_draw() {
            return Ok(());
        }
        let spectators = &o!().cheats.get::<Visuals>(Visuals::name()).spectators;
        let mut y = 100;
        for (name, mode) in &*spectators.lock().unwrap() {
            let text = format!("[{}] {}", mode.to_string(), name);
            frame.text(
                &text,
                frame.window_size().0 / 2,
                y,
                FontSize::Medium,
                true,
                FOREGROUND,
                255,
            );
            let text_size = frame.fonts.get_text_size(&text, FontSize::Medium);
            y += text_size.1 + text_size.2;
        }

        Ok(())
    }
}
