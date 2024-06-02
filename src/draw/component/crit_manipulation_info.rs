use crate::{
    d,
    draw::{
        colors::{BACKGROUND, FOREGROUND, GREEN, RED, WHITE, YELLOW},
        event::Event,
        fonts::FontSize,
        frame::Frame,
    },
    error::OxideResult,
    get_cheat,
    oxide::cheat::crit_manipulation::{CritManipulation, CritManipulationState},
    sdk::entity::player::Player,
};

use super::{
    base::{visible_window::VisibleWindow, window::Window},
    Component, ComponentBase,
};

#[derive(Debug)]
pub struct CritManipulationInfoWindow {
    visible_window: VisibleWindow,
}

#[derive(Debug)]
pub struct CirtManipulationInfo {
    base: ComponentBase,
}
impl CirtManipulationInfo {
    pub fn new() -> CirtManipulationInfo {
        let base = ComponentBase {
            x: 0,
            y: 0,
            w: 200,
            h: 30,
        };
        CirtManipulationInfo { base }
    }
}

impl Component for CirtManipulationInfo {
    fn get_base(&mut self) -> &mut ComponentBase {
        &mut self.base
    }
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let Some(CritManipulationState{crits, max_crits,needed_damage, next_check,crit_time, needed_blanks}) = &get_cheat!(CritManipulation).state else {return Ok(())};

        frame.filled_rect(
            self.base.x,
            self.base.y,
            self.base.w,
            self.base.h,
            BACKGROUND,
            100,
        );
        let (value, color) = if let Some(crit_time) = crit_time {
            (*crit_time, GREEN)
        } else if let Some(next_check) = next_check {
            (*next_check, YELLOW)
        } else {
            (0.0, WHITE)
        };
        frame.filled_rect(
            self.base.x,
            self.base.y,
            (self.base.w as f32 * value) as isize,
            self.base.h,
            color,
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
        let (text, color) = if let Some(damage) = needed_damage {
            (format!("deal {} damage", damage.round()), RED)
        } else if let Some(shots) = needed_blanks {
            let suffix = if *shots > 1 { "times" } else { "time" };
            (format!("fire {} {}", shots, suffix), RED)
        } else {
            (format!("{}/{}", crits, max_crits), WHITE)
        };

        frame.text(
            &text,
            self.base.x + self.base.w / 2,
            self.base.y + self.base.h / 2,
            FontSize::Small,
            true,
            true,
            color,
            255,
        );

        Ok(())
    }
}

impl CritManipulationInfoWindow {
    pub fn new() -> CritManipulationInfoWindow {
        let mut window = Window::new("CRIT MANIPULATION".to_string(), None);
        window.get_base().x = (d!().window_size.0 - window.get_base().w) / 2;
        window.get_base().y = (d!().window_size.1 - window.get_base().y) / 2 + 100;

        let crit_manipulatino_info = CirtManipulationInfo::new();
        window.add(crit_manipulatino_info, 0);
        CritManipulationInfoWindow {
            visible_window: VisibleWindow::new(window),
        }
    }
    fn should_draw(&self) -> bool {
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

impl Component for CritManipulationInfoWindow {
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
