use crate::{draw::component::player_list_window::PlayerListInfo, o, util::arcm::Arcm};

use super::Cheat;

#[derive(Debug)]
pub struct PlayerList {
    pub players: Arcm<Vec<PlayerListInfo>>,
}

impl PlayerList {
    pub fn init() -> PlayerList {
        PlayerList {
            players: Arcm::new(Vec::new()),
        }
    }
    pub fn update(&mut self) {
        let mut new_players = Vec::new();

        for resource in o!().player_resource_manager.all() {
            new_players.push(PlayerListInfo::new(resource));
        }
        new_players.sort_by(|a, b| {
            (a.resource.team.clone().as_i32())
                .partial_cmp(&(b.resource.team.clone().as_i32()))
                .unwrap()
        });
        let mut players = self.players.lock().unwrap();
        *players = new_players;
    }
}
impl Cheat for PlayerList {}
