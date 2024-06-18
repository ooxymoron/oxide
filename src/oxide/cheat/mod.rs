use crate::draw::event::Event;

pub mod cheats;

pub mod aimbot;
pub mod crit_manipulation;
pub mod movement;
pub mod player_list;
pub mod spread_reduction;
pub mod triggerbot;
pub mod visual;

pub trait Cheat: std::fmt::Debug {
    fn handle_event(&mut self, _: &mut Event) {}
}
