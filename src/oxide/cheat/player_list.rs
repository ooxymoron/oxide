use std::collections::HashMap;

use crate::{
    o,
    oxide::player_resource_manager::PlayerResourceData,
    sdk::entity::{player::Player, Team},
    util::arcm::Arcm,
};

use super::Cheat;

#[derive(Debug)]
pub struct PlayerList {
    pub players: Arcm<HashMap<i32, PlayerListInfo>>,
}

#[derive(Debug, Clone)]
pub struct PlayerListInfo {
    pub prio: Arcm<isize>,
    pub name: String,
    pub team: Team,
    pub user_id: i32,
    pub inner: PlayerListInfoInner,
    pub changed: bool,
    pub connected: bool,
}
#[derive(Debug, Clone)]
pub enum PlayerListInfoInner {
    Real {
        guid: String,
        tags: Option<Arcm<(bool, Vec<String>)>>,
    },
    Bot {},
}
impl PlayerListInfoInner {
    pub fn new(resource: &PlayerResourceData) -> PlayerListInfoInner {
        let guid = if let Ok(player) = Player::get_from_user_id(resource.user_id) {
            let info = player.info().unwrap();

            if info.fakeplayer {
                return PlayerListInfoInner::Bot {};
            }

            info.guid
        } else {
            "".to_string()
        };
        if resource.connected && !guid.is_empty() && !resource.name.is_empty() {
            o!().player_db
                .add_player_if_doesnt_exist(&guid, &resource.name);
        }
        let tags = o!()
            .player_db
            .get_player_tags(&guid)
            .map(|x| Arcm::new((true, x)));
        PlayerListInfoInner::Real { guid, tags }
    }
    pub fn update(&mut self, resource: &PlayerResourceData) -> bool {
        let PlayerListInfoInner::Real { guid, .. } = self else {return false};
        *guid
            != if let Ok(player) = Player::get_from_user_id(resource.user_id) {
                let info = player.info().unwrap();

                info.guid
            } else {
                "".to_string()
            }
    }
}

impl PlayerListInfo {
    pub fn new(resource: PlayerResourceData) -> PlayerListInfo {
        PlayerListInfo {
            name: resource.name.clone(),
            team: resource.team.clone(),
            prio: Arcm::new(0),
            user_id: resource.user_id.clone(),
            connected: resource.connected.clone(),
            inner: PlayerListInfoInner::new(&resource),
            changed: false,
        }
    }
    pub fn update(&mut self, resource: &PlayerResourceData) {
        if (!self.connected && resource.connected)
            || self.name != resource.name
            || self.team != resource.team
            || self.inner.update(resource)
        {
            self.name = resource.name.clone();
            self.team = resource.team.clone();
            self.connected = resource.connected;
            self.inner = PlayerListInfoInner::new(resource);
            self.changed = true
        }
    }
}

impl PlayerList {
    pub fn init() -> PlayerList {
        PlayerList {
            players: Arcm::new(HashMap::new()),
        }
    }
    pub fn update(&mut self) {
        let mut players = self.players.lock().unwrap();

        let resources = o!().player_resource_manager.all();
        let valid_ids = resources.iter().map(|x| x.user_id).collect::<Vec<_>>();
        players.retain(|k, _| valid_ids.contains(k));

        for resource in resources {
            if let Some(player) = players.get_mut(&resource.user_id) {
                player.update(&resource);
                if let PlayerListInfo {
                    inner:
                        PlayerListInfoInner::Real {
                            tags: Some(tags),
                            guid,
                            ..
                        },
                    ..
                } = player
                {
                    let mut tags = tags.lock().unwrap();
                    if tags.0 {
                        o!().player_db.set_player_tags(guid, &tags.1)
                    }
                    tags.0 = false
                }
                continue;
            }
            players.insert(resource.user_id.clone(), PlayerListInfo::new(resource));
        }
    }
}
impl Cheat for PlayerList {}
