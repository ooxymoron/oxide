use derivative::Derivative;
use sqlite::Connection;

use crate::util::dir;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct PlayerDb {
    #[derivative(Debug = "ignore")]
    conn: Connection,
}

impl PlayerDb {
    pub fn init() -> PlayerDb {
        let conn = sqlite::open(format!("{}/players.sqlite", dir())).unwrap();
        let query = "
           CREATE TABLE IF NOT EXISTS player (
              guid TEXT PK,
              name TEXT
            );
            CREATE TABLE IF NOT EXISTS tag (id INT PK AUTO_INCREMENT, name TEXT);
            CREATE TABLE IF NOT EXISTS player_tag(
              tag INT,
              player TEXT,
              FOREIGN KEY(player) REFERENCES player(guid),
              FOREIGN KEY(tag) REFERENCES tag(id)
            );
        ";
        conn.execute(query).unwrap();
        conn.iterate("SELECT COUNT(name) FROM player", |pairs| {
            for &(name, value) in pairs {
                dbg!(name, value);
            }
            true
        })
        .unwrap();
        PlayerDb { conn }
    }
}
