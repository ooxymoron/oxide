use crate::{
    d,
    draw::{
        colors::{BACKGROUND, FOREGROUND},
        event::Event,
        fonts::FontSize,
        frame::Frame,
    },
    error::OxideResult,
    get_cheat,
    oxide::cheat::spread_reduction::SpreadReduction,
    sdk::entity::player::Player,
    setting,
};

use super::{
    base::{visible_window::VisibleWindow, window::Window},
    Component, ComponentBase,
};

const PAD: isize = 6;

#[derive(Debug)]
pub struct SpreadReductionInfoWindow {
    visible_window: VisibleWindow,
}

#[derive(Debug)]
pub struct SpreadReductionInfo {
    base: ComponentBase,
}
impl SpreadReductionInfo {
    pub fn new() -> SpreadReductionInfo {
        let base = ComponentBase {
            x: 0,
            y: 0,
            w: 300,
            h: 70,
        };
        SpreadReductionInfo { base }
    }
}

impl Component for SpreadReductionInfo {
    fn get_base(&mut self) -> &mut ComponentBase {
        &mut self.base
    }
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let text = get_cheat!(SpreadReduction).state.to_string();
        frame.filled_rect(
            self.base.x,
            self.base.y,
            self.base.w,
            self.base.h,
            BACKGROUND,
            100,
        );
        frame.outlined_rect(
            self.base.x,
            self.base.y,
            self.base.w,
            self.base.h,
            FOREGROUND,
            200,
        );

        let mut y = self.base.y + PAD + 8;
        for line in text.split("\n") {
            frame.text(
                &line,
                self.base.x + PAD,
                y,
                FontSize::Small,
                false,
                FOREGROUND,
                255,
            );
            let text_size = frame.fonts.get_text_size(&line, FontSize::Small);
            y += text_size.1 + text_size.2 + 3;
        }

        Ok(())
    }
}

impl SpreadReductionInfoWindow {
    pub fn new() -> SpreadReductionInfoWindow {
        let mut window = Window::new("SPREAD REDUCTION".to_string(), None);
        let spectator_list = SpreadReductionInfo::new();

        window.get_base().x = 10;
        window.get_base().y = (d!().window_size.1 - window.get_base().y) / 2;

        window.add(spectator_list, 0);

        SpreadReductionInfoWindow {
            visible_window: VisibleWindow::new(window),
        }
    }
    fn should_draw(&self) -> bool {
        if !setting!(aimbot, spread_reduction) {
            return false;
        }
        if Player::get_local().is_err() {
            return false;
        };
        return true;
    }
    pub fn draw_wrapper(&mut self, frame: &mut Frame, visible: bool) -> OxideResult<()> {
        if !self.should_draw() {
            return Ok(());
        }
        if !visible {
            self.visible_window.draw_hidden(frame)?;
            return Ok(());
        }
        self.draw(frame)
    }
}

impl Component for SpreadReductionInfoWindow {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        self.visible_window.draw(frame)
    }

    fn get_base(&mut self) -> &mut super::ComponentBase {
        self.visible_window.get_base()
    }
    fn handle_event(&mut self, event: &mut Event) {
        if !self.should_draw() {
            return;
        }
        self.visible_window.handle_event(event)
    }
}
