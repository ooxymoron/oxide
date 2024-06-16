use crate::{
    d,
    draw::{colors::FOREGROUND, event::Event, frame::Frame},
    error::OxideResult,
    o,
    oxide::cheat::visual::Visuals,
    sdk::entity::player::Player,
    setting,
};

use super::{
    base::{
        label::Label,
        linear_layout::{LinearLayout, LinearLayoutOrientation},
        visible_window::VisibleWindow,
        window::Window,
    },
    Component, ComponentBase,
};

#[derive(Debug)]
pub struct SpectatorListWindow {
    visible_window: VisibleWindow,
}

#[derive(Debug)]
pub struct SpectatorList {
    base: ComponentBase,
}
impl SpectatorList {
    pub fn new() -> SpectatorList {
        let base = ComponentBase {
            x: 0,
            y: 0,
            w: 0,
            h: 40,
        };
        SpectatorList { base }
    }
}

impl Component for SpectatorList {
    fn get_base(&mut self) -> &mut ComponentBase {
        &mut self.base
    }
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let spectators = &o!().cheats.get::<Visuals>().spectators;
        let mut container = LinearLayout::new(LinearLayoutOrientation::VERTICAL, 8, 10);
        container.center = true;

        for (name, mode, team) in &*spectators.lock().unwrap() {
            let mut row = LinearLayout::new(LinearLayoutOrientation::HORIZONTAL, 8, 0);
            let mode = format!("[{}]", mode.to_string());
            row.add(Label::new(mode, 0, 0, FOREGROUND));
            row.add(Label::new(name.to_string(), 0, 0, team.color()));
            container.add(row);
        }
        container.get_base().x = self.get_base().x + (self.base.w - container.get_base().w) / 2;
        container.get_base().y = self.get_base().y;
        container.draw(frame)
    }
}

impl SpectatorListWindow {
    pub fn new() -> SpectatorListWindow {
        let mut window = Window::new("SPECTATOR LIST".to_string(), None);
        let mut spectator_list = SpectatorList::new();
        spectator_list.get_base().w = window.get_base().w;
        window.get_base().x = (d!().window_size.0 - window.get_base().w) / 2;

        window.add(spectator_list);

        SpectatorListWindow {
            visible_window: VisibleWindow::new(window),
        }
    }
    fn should_draw(&self) -> bool {
        if !*setting!(visual, spectator_list) {
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

impl Component for SpectatorListWindow {
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
