use std::{
    any::type_name, collections::HashMap, mem::{transmute, ManuallyDrop}
};

use crate::{draw::event::Event, oxide::cheat::{crit_manipulation::CritManipulation, player_list::PlayerList, spread_reduction::SpreadReduction, triggerbot::Triggerbot}};

use super::{aimbot::Aimbot, movement::Movement, visual::Visuals, Cheat};

#[derive(Debug)]
pub struct Cheats(pub HashMap<String, Box<dyn Cheat>>);

impl Cheats {
    pub fn init() -> Cheats {
        let cheats = HashMap::new();
        let mut cheats = Cheats(cheats);

        macro_rules! add {
            ($cheat: ident) => {
                {
                    cheats.add($cheat::init(), type_name::<$cheat>());
                }
            };
        }

        add!(Aimbot);
        add!(Movement);
        add!(Visuals);
        add!(SpreadReduction);
        add!(CritManipulation);
        add!(PlayerList);
        add!(Triggerbot);

        cheats
    }
    pub fn handle_event(&mut self, event: &mut Event) {
        for (_, cheat) in &mut self.0 {
            cheat.handle_event(event)
        }
    }
    fn add(&mut self, cheat: impl Cheat + 'static, name: &str) {
        self.0.insert(name.to_owned(), Box::new(cheat));
    }
    pub fn get<T>(&mut self) -> ManuallyDrop<&mut Box<T>> {
        unsafe { ManuallyDrop::new(transmute(self.0.get_mut(type_name::<T>()).unwrap())) }
    }
}
