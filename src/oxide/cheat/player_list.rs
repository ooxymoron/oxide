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
    pub players: Arcm<(bool, HashMap<i32, PlayerListInfo>)>,
}

#[derive(Debug, Clone)]
pub struct PlayerListInfo {
    pub prio: Arcm<(bool, Arcm<isize>)>,
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
        tags: Option<Arcm<(bool, Arcm<Vec<String>>)>>,
    },
    Bot {},
}
impl PlayerListInfoInner {
    pub fn new(guid: Option<String>) -> PlayerListInfoInner {
        let Some(guid) = guid else {
            return PlayerListInfoInner::Bot {  }
        };
        let tags = o!()
            .player_db
            .get_player_tags(&guid)
            .map(|x| Arcm::new((true, Arcm::new(x))));
        PlayerListInfoInner::Real {
            guid: guid.to_string(),
            tags,
        }
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
        let guid = if let Ok(player) = Player::get_from_user_id(resource.user_id) {
            player.get_guid()
        } else {
            None
        };
        let prio = if let Some(guid) = &guid {
            if !resource.name.is_empty() && resource.name != "unconnected" {
                o!().player_db
                    .add_player_if_doesnt_exist(&guid, &resource.name);
            }
            o!().player_db.get_player_prio(&guid).unwrap_or(0)
        } else {
            0
        };
        PlayerListInfo {
            name: resource.name.clone(),
            team: resource.team.clone(),
            prio: Arcm::new((false, Arcm::new(prio))),
            user_id: resource.user_id.clone(),
            connected: resource.connected.clone(),
            inner: PlayerListInfoInner::new(guid),
            changed: false,
        }
    }
    pub fn update(&mut self, resource: &PlayerResourceData) {
        let guid = if let Ok(player) = Player::get_from_user_id(resource.user_id) {
            player.get_guid()
        } else {
            None
        };
        if (!self.connected && resource.connected)
            || self.name != resource.name
            || self.team != resource.team
            || self.inner.update(resource)
        {
            self.name = resource.name.clone();
            self.team = resource.team.clone();
            self.connected = resource.connected;
            if let Some(guid) = &guid {
                let prio = self.prio.lock().unwrap();
                *prio.1.lock().unwrap() = o!().player_db.get_player_prio(guid).unwrap_or(0);
            }
            self.inner = PlayerListInfoInner::new(guid);
            self.changed = true
        }
    }
}

impl PlayerList {
    pub fn init() -> PlayerList {
        PlayerList {
            players: Arcm::new((true, HashMap::new())),
        }
    }
    pub fn update(&mut self) {
        let mut changed = false;
        let mut players = self.players.lock().unwrap();

        let resources = o!().player_resource_manager.all();
        let valid_ids = resources.iter().map(|x| x.user_id).collect::<Vec<_>>();
        players.1.retain(|k, _| {
            let retain = valid_ids.contains(k);
            if !retain {
                changed = true;
            }
            retain
        });

        for resource in resources {
            if let Some(player) = players.1.get_mut(&resource.user_id) {
                player.update(&resource);
                if let PlayerListInfo {
                    prio,
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
                        dbg!("updateing tags");
                        o!().player_db
                            .set_player_tags(guid, &tags.1.lock().unwrap());
                    }
                    tags.0 = false;
                    let mut prio = prio.lock().unwrap();
                    if prio.0 {
                        dbg!("updateing prio");

                        o!().player_db
                            .set_player_prio(guid, *prio.1.lock().unwrap());
                    }
                    prio.0 = false
                }
                continue;
            }
            changed = true;
            players
                .1
                .insert(resource.user_id.clone(), PlayerListInfo::new(resource));
        }
        players.0 = changed;
    }
}
impl Cheat for PlayerList {}
