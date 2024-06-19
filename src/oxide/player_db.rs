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
            guid TEXT PK NOT NULL,
            name TEXT NOT NULL,
            prio INT NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS tag (name TEXT PK NOT NULL);
        CREATE TABLE IF NOT EXISTS player_tag(
            tag TEXT NOT NULL,
            player TEXT NOT NULL,
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

                ('pooper');


             INSERT INTO player(name, guid) VALUES
             ('anthony','[U:1:1286352511]'),
             ('retard tom guy','[U:1:1418751454]'),
             ('oxy','[U:1:195860616]'),
             ('pooper','[U:1:1286352511]');

             INSERT INTO player_tag(player, tag) VALUES
             ('[U:1:1418751454]','closet'),
             ('[U:1:1418751454]','faggot'),
             ('[U:1:1418751454]','furry'),
             ('[U:1:1418751454]','skid'),
             ('[U:1:1418751454]','paster'),
             ('[U:1:1418751454]','bot'),
             ('[U:1:1418751454]','pedo'),
             ('[U:1:195860616]','oxide'),
             ('[U:1:195860616]','cheater'),
             ('[U:1:195860616]','rage cheater');
             ('[U:1:1286352511]','pooper');
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
    const ADD_PLAYER_QUERY: &'static str = "INSERT INTO player(name, guid) VALUES(:name,:guid);";
    pub fn add_player_if_doesnt_exist(&self, guid: &str, name: &str) -> bool {
        let exists = self.does_player_exist(guid);
        if !exists {
            self.conn
                .prepare(Self::ADD_PLAYER_QUERY)
                .unwrap()
                .into_iter()
                .bind(&[(":name", name), (":guid", guid)][..])
                .unwrap();
        }
        !exists
    }
    const DOES_PLAYER_EXIST_QUERY: &'static str = "SELECT * FROM player WHERE guid = :guid;";
    pub fn does_player_exist(&self, guid: &str) -> bool {
        let res = self
            .conn
            .prepare(Self::DOES_PLAYER_EXIST_QUERY)
            .unwrap()
            .into_iter()
            .bind((":guid", guid))
            .unwrap()
            .next()
            .is_some();

        res
    }
    const GET_PLAYER_TAGS_QUERY: &'static str = "
                    SELECT tag.name 
                    FROM player 	
                    INNER JOIN player_tag 
                        on player.guid = player_tag.player 
                    INNER JOIN tag 
                        ON tag.name = player_tag.tag
                    WHERE player.guid = :guid;
";
    pub fn get_player_tags(&self, guid: &str) -> Option<Vec<String>> {
        if !self.does_player_exist(guid) {
            return None;
        }
        let mut tags = Vec::new();

        for row in self
            .conn
            .prepare(Self::GET_PLAYER_TAGS_QUERY)
            .unwrap()
            .into_iter()
            .bind((":guid", guid))
            .unwrap()
            .map(|x| x.unwrap())
        {
            tags.push(row.read::<&str, _>("name").to_string());
        }

        Some(tags)
    }
    const GET_PLAYER_PRIO_QUERY: &'static str = "SELECT prio FROM player WHERE guid = :guid;";
    pub fn get_player_prio(&self, guid: &str) -> Option<isize> {
        if !self.does_player_exist(guid) {
            return None;
        }
        let prio = self
            .conn
            .prepare(Self::GET_PLAYER_PRIO_QUERY)
            .unwrap()
            .into_iter()
            .bind((":guid", guid))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .read::<i64, _>("prio") as isize;
        Some(prio)
    }

    const DELETE_PLAYER_TAGS_QUERY: &'static str = "DELETE FROM player WHERE player.guid = :guid;";
    const INSERT_PLAYER_TAG_QUERY: &'static str =
        "INSERT INTO player_tag(player,tag) VALUES (:player,:tag);";
    pub fn set_player_tags(&self, guid: &str, tags: &Vec<String>) {
        self.conn
            .prepare(Self::DELETE_PLAYER_TAGS_QUERY)
            .unwrap()
            .into_iter()
            .bind((":guid", guid))
            .unwrap();

        for tag in tags {
            self.conn
                .prepare(Self::INSERT_PLAYER_TAG_QUERY)
                .unwrap()
                .into_iter()
                .bind(&[(":player", guid), (":tag", tag)][..])
                .unwrap();
        }
    }

    const GET_TAGS_QUERY: &'static str = "SELECT tag.name FROM tag;";
    pub fn get_tags(&self) -> Vec<String> {
        let mut tags = Vec::new();

        for row in self
            .conn
            .prepare(Self::GET_TAGS_QUERY)
            .unwrap()
            .into_iter()
            .map(|x| x.unwrap())
        {
            tags.push(row.read::<&str, _>("name").to_string())
        }
        tags
    }
    const UPDATE_PRIO_QUERY: &'static str = "UPDATE player SET prio = :prio WHERE guid = :guid;";
    pub fn set_player_prio(&self, guid: &str, prio: isize) {
        self.conn
            .prepare(Self::UPDATE_PRIO_QUERY)
            .unwrap()
            .into_iter()
            .bind(&[(":prio", prio.to_string().as_str()), (":guid", guid)][..])
            .unwrap();
    }
}
