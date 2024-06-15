use std::cmp::Ordering;

use crate::error::OxideResult;

use super::{event::Event, frame::Frame};

pub mod aimbot_fov;
pub mod base;
pub mod overlay;
pub mod visuals_window;
pub mod aimbot_window;
pub mod movement_window;
pub mod spectator_list;
pub mod spread_reduction_info;
pub mod crit_manipulation_info;
pub mod player_list_window;
pub mod crit_manipulation_window;
pub mod spread_reduction_window;

#[derive(Debug,Clone)]
pub enum DrawOrder {
    Top,
    Value(usize)
}

#[derive(Debug,Clone)]
pub struct ComponentBase {
    pub x: isize,
    pub y: isize,
    pub w: isize,
    pub h: isize
}

#[allow(unused)]
pub trait Component: std::fmt::Debug {
    fn get_base(&mut self) -> &mut ComponentBase;
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()>{Ok(())}
    fn handle_event(&mut self, event: &mut Event){}
    fn get_draw_order(&self) -> DrawOrder {
        DrawOrder::Value(0)
    }
    fn set_draw_order(&mut self, order: DrawOrder){
        ()
    }
}

#[derive(Debug)]
pub struct Components(pub Vec<Box<dyn Component>>);

impl Components {
    pub fn new() -> Components {
        Components(Vec::new())
    }
    pub fn add(&mut self, component: impl Component + 'static) {
        self.0.push(Box::new(component));
    }
    pub fn draw(&mut self, frame: &mut Frame) -> OxideResult<()>{
        self.sort();
        for component in &mut self.0 {
            component.draw(frame)?
        }
        Ok(())
    }
    pub fn handle_event(&mut self, event: &mut Event) {

        for component in &mut self.0.iter_mut().rev(){
            if event.handled {
                break;
            }
            component.handle_event(event)
        }
    }
    fn sort(&mut self) {
        self.0.sort_by(|a,b|{
            let a = a.get_draw_order();
            let b = b.get_draw_order();
            let DrawOrder::Value(a) = a else {
                return Ordering::Greater
            };
            let DrawOrder::Value(b) = b else {
                return Ordering::Less
            };
            a.cmp(&b)
        });
        for (i,component) in self.0.iter_mut().enumerate() {
            component.set_draw_order(DrawOrder::Value(i))
        }

    }
}
