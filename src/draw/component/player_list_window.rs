use crate::{
    draw::{colors::FOREGROUND, event::Event, frame::Frame},
    error::OxideResult,
    o,
    oxide::{cheat::player_list::PlayerList, player_resource_manager::PlayerResourceData},
    sdk::{entity::player::Player, interfaces::base_engine::PlayerInfo},
    util::arcm::Arcm,
};

use super::{
    base::{label::Label, table::Table, window::Window},
    Component, ComponentBase,
};

#[derive(Debug, Clone)]
pub struct PlayerListInfo {
    pub resource: PlayerResourceData,
    pub info: Option<PlayerInfo>,
}

impl PlayerListInfo {
    pub fn new(resource: PlayerResourceData) -> PlayerListInfo {
        let info = if let Ok(player) = Player::get_from_user_id(resource.user_id) {
            Some(player.info().unwrap())
        } else {
            None
        };
        PlayerListInfo { resource, info }
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

        let player_list = o!().cheats.get::<PlayerList>();
        let players = player_list.players.lock().unwrap().clone();
        let mut table_data = Vec::new();

        table_data.push([
            Box::new(Label::new("name".to_string(), 0, 0, FOREGROUND)) as Box<dyn Component>,
            Box::new(Label::new("guid".to_string(), 0, 0, FOREGROUND)) as Box<dyn Component>,
        ]);
        for player in players {
            let guid = if let Some(info) = player.info {
                info.guid
            } else {
                "".to_string()
            };
            let mut name = Label::new(player.resource.name, 0, 0, player.resource.team.color());
            name.copy = true;
            let mut guid = Label::new(guid, 0, 0, FOREGROUND);
            guid.copy = true;
            table_data.push([
                Box::new(name) as Box<dyn Component>,
                Box::new(guid) as Box<dyn Component>,
            ]);
        }

        let table = Table::new(table_data);

        self.window.add(table);

        self.window.draw(frame)?;
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
