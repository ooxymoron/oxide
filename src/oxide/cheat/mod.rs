use crate::draw::event::Event;


pub mod cheats;

pub mod aimbot;
pub mod movement;
pub mod visual;
pub mod spread_reduction;

pub trait Cheat: std::fmt::Debug {
    fn handle_event(&mut self, _: &mut Event) {}
}
