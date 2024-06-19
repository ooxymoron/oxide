use std::{collections::HashMap, mem::transmute};

use crate::{
    draw::{colors::FOREGROUND, event::Event, frame::Frame},
    error::OxideResult,
    o,
    oxide::cheat::player_list::{PlayerList, PlayerListInfo, PlayerListInfoInner},
    util::arcm::Arcm,
};

use super::{
    base::{int_input::IntInput, label::Label, select::Select, table::Table, window::Window},
    Component, ComponentBase,
};

#[derive(Debug)]
pub struct PlayerListWindow {
    window: Window,
    last_players: Option<HashMap<i32, PlayerListInfo>>,
}

impl PlayerListWindow {
    pub fn new(visible: Arcm<bool>) -> PlayerListWindow {
        let headers = [
            Box::new(Label::new("name".to_string(), 0, 0, FOREGROUND)) as Box<dyn Component>,
            Box::new(Label::new("guid".to_string(), 0, 0, FOREGROUND)) as Box<dyn Component>,
            Box::new(Label::new("prio".to_string(), 0, 0, FOREGROUND)) as Box<dyn Component>,
            Box::new(Label::new("tags".to_string(), 0, 0, FOREGROUND)) as Box<dyn Component>,
        ];
        let mut window = Window::new("PLAYER LIST".to_string(), Some(visible));
        window.add(Table::new(headers, HashMap::new()));
        PlayerListWindow {
            window,
            last_players: None,
        }
    }
    pub fn update_data(&mut self) {
        let player_list = o!().cheats.get::<PlayerList>();
        let mut players = player_list.players.lock().unwrap();

        let table: &mut Box<Table<4>> =
            unsafe { transmute(self.window.components.0.first_mut().unwrap()) };

        table.data.retain(|id, _| {
            let Some(player) = players.1.get_mut(id) else { return false};
            if player.changed {
                player.changed = false;
                return false;
            }
            true
        });
        let tags = o!().player_db.get_tags();

        for (id, player) in players.1.iter() {
            if table.data.contains_key(id) {
                continue;
            }
            let mut name = Label::new(player.name.clone(), 0, 0, player.team.color());
            name.copy = true;
            let (guid, player_tags) = if let PlayerListInfo {
                inner: PlayerListInfoInner::Real { guid, tags, .. },
                ..
            } = player
            {
                (guid.clone(), tags.clone())
            } else {
                ("".to_string(), None)
            };
            let mut guid = Label::new(guid, 0, 0, FOREGROUND);
            guid.copy = true;
            let mut prio = IntInput::new(0, 0, None, player.prio.clone(), None);
            prio.text_input.background = false;
            let tags = player_tags
                .map(|player_tags| {
                    Box::new(Select::new(tags.clone(), player_tags.clone(), true, None))
                        as Box<dyn Component>
                })
                .unwrap_or(
                    Box::new(Label::new("".to_string(), 0, 0, FOREGROUND)) as Box<dyn Component>
                );

            table.data.insert(
                *id,
                [
                    Box::new(name) as Box<dyn Component>,
                    Box::new(guid) as Box<dyn Component>,
                    Box::new(prio) as Box<dyn Component>,
                    tags,
                ],
            );
        }

        self.last_players = Some(players.1.clone());
        table.update_data();
        self.window.update_size();
    }
}

impl Component for PlayerListWindow {
    fn get_base(&mut self) -> &mut ComponentBase {
        self.window.get_base()
    }
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        self.update_data();

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
