use std::mem::transmute;

use derivative::Derivative;

use crate::{cfn, vmt_call};

use super::{ids::ItemDefinitionInex, Weapon};


#[repr(C)]
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct VMTGun {
    #[derivative(Debug = "ignore")]
    _pad: [usize; 537],
    pub get_projectile_spread: cfn!(f32, &Gun), //0x87c
    _pad1: [usize; 5],
    pub get_projectile_damage: cfn!(f32, &Gun), //0x87c
                                                //0x8A4
}

#[repr(C)]
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct Gun {
    pub vmt: *mut VMTGun,
}

impl Gun {
    pub fn as_weapon(&mut self) -> &'static mut Weapon {
        return unsafe { transmute(self) };
    }

    pub fn get_damage(&mut self, crit: bool) -> f32 {
        let mut mult = if crit { 3.0 } else { 1.0 };
        if crit
            && matches!(
                self.as_weapon().get_item_definition_index(),
                ItemDefinitionInex::SniperMTheSydneySleeper
            )
        {
            mult = 1.35
        }
        vmt_call!(self, get_projectile_damage) * mult
    }
    pub fn sniper_charge(&mut self) -> f32 {
        (vmt_call!(self, get_projectile_damage) - 50.0) / 100.0
    }
    pub fn get_bullets(&mut self) -> i32 {
        let mode = self.as_weapon().get_mode();
        let mut bullet_count =
            self.as_weapon().get_info().weapon_data[mode as usize].bullets_per_shot;
        if let Some(bullets_attrib) = self
            .as_weapon()
            .as_ent()
            .get_float_attrib("mult_bullets_per_shot")
        {
            bullet_count = bullets_attrib as i32;
        }
        bullet_count
    }
}

