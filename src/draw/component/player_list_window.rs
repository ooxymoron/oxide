use crate::{
    draw::{colors::FOREGROUND, event::Event, frame::Frame},
    error::OxideResult,
    o,
    oxide::{cheat::player_list::PlayerList, player_resource_manager::PlayerResourceData},
    util::arcm::Arcm,
};

use super::{
    base::{
        label::Label,
        window::{Window, HEADER_HEIGHT},
    },
    Component, ComponentBase,
};

const PADDING: isize = 10;

#[derive(Debug)]
pub struct PlayerListInfo {
    pub resource: PlayerResourceData,
}

impl PlayerListInfo {
    pub fn new(resource: PlayerResourceData) -> PlayerListInfo {
        PlayerListInfo { resource }
    }
}
#[derive(Debug)]
pub struct PlayerListWindow {
    window: Window,
}

impl PlayerListWindow {
    pub fn new(visible: Arcm<bool>) -> PlayerListWindow {
        PlayerListWindow {
            window: Window::new("PLAYER LIST".to_string(), Some(visible)),
        }
    }
}

impl Component for PlayerListWindow {
    fn get_base(&mut self) -> &mut ComponentBase {
        self.window.get_base()
    }
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        self.window.clear();
        let players = &o!().cheats.get::<PlayerList>().players;

        let mut y = PADDING / 2;
        let x = PADDING;
        let mut line_ys = Vec::new();

        for (i, player) in players.lock().unwrap().iter().enumerate() {
            let pr = &player.resource;
            let text = format!("{} | {}", pr.name, pr.id);
            let mut label = Label::new(text, x, y, pr.team.color());
            label.get_base().w += PADDING;
            if i != 0 {
                let base = self.window.get_base();

                let y = y - PADDING / 2 + base.y + HEADER_HEIGHT;
                line_ys.push(y)
            }
            y += PADDING + label.get_base().h;

            self.window.add(label, PADDING);
        }
        self.window.get_base().h -= PADDING / 2;
        self.window.draw(frame)?;
        let base = self.window.get_base().clone();
        for y in line_ys {
            if self.window.should_draw() {
                frame.line(base.x, y, base.x + base.w, y, FOREGROUND, 255);
            }
        }
        Ok(())
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
}
