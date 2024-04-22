use crate::draw::event::Event;


pub mod cheats;

pub mod aimbot;
pub mod movement;
pub mod visual;

pub trait Cheat: std::fmt::Debug {
    fn handle_event(&mut self, event: &mut Event);
}
