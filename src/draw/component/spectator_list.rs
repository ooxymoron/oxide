use std::borrow::BorrowMut;

use crate::{
    d, draw::{colors::FOREGROUND, fonts::FontSize}, error::OxideResult, o, oxide::cheat::visual::Visuals, sdk::entity::Entity, setting
};

use super::{Component, ComponentBase};

#[derive(Debug)]
pub struct SpectatorList {
    base: ComponentBase
}

impl SpectatorList {
    pub fn new() -> SpectatorList {
        SpectatorList {
            base: ComponentBase{x:100,y:100,w: 100,h:100}
        }
    }
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
                d!().window_size.0 / 2,
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


    fn get_base(&mut self) -> &mut super::ComponentBase {
        self.base.borrow_mut()
    }
}
