use std::path::Path;

use derivative::Derivative;
use sqlite::Connection;

use crate::{log, util::dir};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct PlayerDb {
    #[derivative(Debug = "ignore")]
    conn: Connection,
}

impl PlayerDb {
    pub fn init() -> PlayerDb {
        let db_file = format!("{}/players.sqlite", dir());

        let fresh = !Path::new(&db_file).exists();
        let conn = sqlite::open(db_file).unwrap();
        let query = "
        CREATE TABLE IF NOT EXISTS player (
            guid TEXT PK,
            name TEXT
        );
        CREATE TABLE IF NOT EXISTS tag (name TEXT PK);
        CREATE TABLE IF NOT EXISTS player_tag(
            tag TEXT,
            player TEXT,
            FOREIGN KEY(player) REFERENCES player(guid),
            FOREIGN KEY(tag) REFERENCES tag(name)
        );
";
        conn.execute(query).unwrap();
        if fresh {
            let query = "
            INSERT INTO tag (name) VALUES 
                ('cheater'),
                ('closet'),
                ('rage cheater'),
                ('faggot'),
                ('pedo'),
                ('furry'),
                ('skid'),
                ('paster'),
                ('bot'),

                ('ratjin'),
                ('fagbox'),
                ('fedware'),
                ('nullcore'),
                ('oxide'),
                ('monkeybot'),

                ('pooper'),
;


             INSERT INTO player(name, guid) VALUES('anthony','[U:1:1286352511]');
             INSERT INTO player_tag(player, tag) VALUES('[U:1:1418751454]','pooper');

             INSERT INTO player(name, guid) VALUES('retard tom guy','[U:1:1418751454]');
             INSERT INTO player_tag(player, tag) VALUES('[U:1:1418751454]','closet');
             INSERT INTO player_tag(player, tag) VALUES('[U:1:1418751454]','faggot');
             INSERT INTO player_tag(player, tag) VALUES('[U:1:1418751454]','furry');
             INSERT INTO player_tag(player, tag) VALUES('[U:1:1418751454]','skid');
             INSERT INTO player_tag(player, tag) VALUES('[U:1:1418751454]','paster');
             INSERT INTO player_tag(player, tag) VALUES('[U:1:1418751454]','bot');
             INSERT INTO player_tag(player, tag) VALUES('[U:1:1418751454]','pedo');

             INSERT INTO player(name, guid) VALUES('oxy','[U:1:195860616]');
             INSERT INTO player_tag(player, tag) VALUES('[U:1:195860616]','oxide');
             INSERT INTO player_tag(player, tag) VALUES('[U:1:195860616]','cheater');
             INSERT INTO player_tag(player, tag) VALUES('[U:1:195860616]','rage cheater');
            ";
            conn.execute(query).unwrap();
        }
        conn.iterate("SELECT COUNT(name) FROM player", |pairs| {
            for &(_, value) in pairs {
                log!("loaded {} players", value.unwrap());
            }
            true
        })
        .unwrap();
        PlayerDb { conn }
    }
    pub fn add_player_if_doesnt_exist(&self, guid: &str, name: &str) -> bool {
        let exists = self.does_player_exist(guid);
        if !exists {
            log!("adding player to db ({},{})", name, guid);
            self.conn
                .execute(format!(
                    "INSERT INTO player(name, guid) VALUES('{}','{}');",
                    name, guid
                ))
                .unwrap();
        }
        !exists
    }
    pub fn does_player_exist(&self, guid: &str) -> bool {
        let mut exists = false;
        self.conn
            .iterate(
                format!(
                    "
                    SELECT * FROM player
                    WHERE player.guid = '{}'",
                    guid
                ),
                |_| {
                    exists = true;
                    true
                },
            )
            .unwrap();
        exists
    }
    pub fn get_player_tags(&self, guid: &str) -> Option<Vec<String>> {
        if !self.does_player_exist(guid) {
            return None;
        }
        let mut tags = Vec::new();

        self.conn
            .iterate(
                format!(
                    "
                    SELECT tag.name 
                    FROM player 	
                    INNER JOIN player_tag 
                        on player.guid = player_tag.player 
                    INNER JOIN tag 
                        ON tag.name = player_tag.tag
                    WHERE player.guid = '{}'
                    ",
                    guid
                ),
                |pairs| {
                    for &(_, value) in pairs {
                        tags.push(value.unwrap().to_string());
                    }
                    true
                },
            )
            .unwrap();
        Some(tags)
    }
    pub fn set_player_tags(&self, guid: &str, tags: &Vec<String>) {
        if tags.is_empty() {
            return;
        }
        self.conn
            .execute(format!(
                "
                    DELETE FROM 
                    player_tag 
                    WHERE player = '{}';

                    INSERT INTO 
                    player_tag(player,tag)
                    VALUES
                    {};
                    ",
                guid,
                tags.into_iter()
                    .map(|tag| format!("('{}','{}')", guid, tag))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .unwrap();
    }
    pub fn get_tags(&self) -> Vec<String> {
        let mut tags = Vec::new();

        self.conn
            .iterate(
                "
                SELECT tag.name
                FROM tag
                ",
                |pairs| {
                    for &(_, value) in pairs {
                        tags.push(value.unwrap().to_string())
                    }
                    true
                },
            )
            .unwrap();
        tags
    }
}
