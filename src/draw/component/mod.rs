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

#[derive(Debug,Clone)]
pub enum DrawOrder {
    Top,
    Value(usize)
}

#[allow(unused)]
pub trait Component: std::fmt::Debug {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) -> OxideResult<()>{Ok(())}
    fn height(&self) -> isize {0}
    fn handle_event(&mut self, event: &mut Event){}
    fn get_draw_order(&self) -> DrawOrder {
        DrawOrder::Value(0)
    }
    fn set_draw_order(&mut self, order: DrawOrder){
        ()
    }
}

#[derive(Debug)]
pub struct Components(Vec<Box<dyn Component>>);

impl Components {
    pub fn new() -> Components {
        Components(Vec::new())
    }
    pub fn add(&mut self, component: impl Component + 'static) {
        self.0.push(Box::new(component));
    }
    pub fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) -> OxideResult<()>{
        self.sort();
        for component in &mut self.0 {
            component.draw(frame, root_x, root_y)?
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
