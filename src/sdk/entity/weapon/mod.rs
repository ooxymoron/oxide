use std::{ffi::CStr, mem::transmute};

use crate::{o, vmt_call};

use self::{
    entity::Entity,
    ids::{ItemDefinitionInex, WeaponId},
    info::WeaponInfo,
    interfaces::condition::ConditionFlags,
};

use super::*;

pub mod crit;
pub mod ids;
pub mod info;
pub mod netvars;

#[repr(C)]
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct VMTWeapon {
    #[derivative(Debug = "ignore")]
    _pad1: [usize; 79],
    pub get_index: cfn!(isize, &Weapon),
    #[derivative(Debug = "ignore")]
    _pad21: [usize; 318],
    pub get_slot: cfn!(isize, &Weapon),
    #[derivative(Debug = "ignore")]
    _pad3: [usize; 1],
    pub get_name: cfn!(CStr, &Weapon),
    #[derivative(Debug = "ignore")]
    _pad4: [usize; 48],
    pub get_weapon_id: cfn!(WeaponId, &Weapon),
    pub get_damage_type: cfn!(isize, &Weapon),
    #[derivative(Debug = "ignore")]
    _pad5: [usize; 14],
    pub calc_is_attack_critical_helper: cfn!(bool, &Weapon),
    #[derivative(Debug = "ignore")]
    _pad6: [usize; 28],
    pub can_fire_critical_shot: cfn!(bool, &Weapon, bool), //0x525
}

#[repr(C)]
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct Weapon {
    pub vmt: *const VMTWeapon,
    _pad: [u8; 0x1ee],
    pub current_seed: i32,
}

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

#[repr(C)]
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct MeleWeapon {
    #[derivative(Debug = "ignore")]
    _pad5: [u8; 0xc2c],
    pub smack_time: f32, /* 0xC2C */
    #[derivative(Debug = "ignore")]
    _pad6: [u8; 0x10],
    pub ready_to_backstab: bool, /*0xC40*/
}

impl Weapon {
    pub fn as_ent(&self) -> &'static mut Entity {
        #[allow(mutable_transmutes)]
        return unsafe { transmute::<_, _>(self) };
    }
    pub fn as_mele(&self) -> &'static mut MeleWeapon {
        #[allow(mutable_transmutes)]
        return unsafe { transmute(self) };
    }
    pub fn as_gun(&mut self) -> OxideResult<&'static mut Gun> {
        if !self
            .as_ent()
            .as_networkable()
            .get_client_class()
            .get_ingeritance_chain()
            .contains(&"CTFWeaponBaseGun".to_string())
        {
            return Err(OxideError::new("this weapon is not a gun"));
        };
        return Ok(unsafe { transmute(self) });
    }
}

impl Weapon {
    pub fn can_attack(&mut self) -> bool {
        let owner = self.get_owner().resolve().unwrap().as_player().unwrap();
        let info = self.get_info();
        let data = &info.weapon_data[self.get_mode()];
        let delay = if info.melee_weapon {
            data.smack_delay
        } else {
            (o!().util.apply_fire_delay)(data.time_fire_delay, self)
        };
        *self.get_last_fire() + delay <= owner.time() && *self.get_clip1() != 0
    }
    pub fn is_sniper_rifle(&mut self) -> bool {
        matches!(
            vmt_call!(self, get_weapon_id),
            WeaponId::Sniperrifle | WeaponId::SniperrifleClassic | WeaponId::SniperrifleDecap
        )
    }
    pub fn is_minigun(&mut self) -> bool {
        matches!(vmt_call!(self, get_weapon_id), WeaponId::Minigun)
    }
    pub fn is_ambassador(&mut self) -> bool {
        matches!(
            self.get_item_definition_index(),
            ItemDefinitionInex::SpyMTheAmbassador | ItemDefinitionInex::SpyMFestiveAmbassador
        )
    }
    pub fn can_headshot(&mut self) -> bool {
        let p_local = Player::get_local().unwrap();
        (self.is_sniper_rifle() && p_local.get_condition().get(ConditionFlags::Zoomed))
            || (self.is_ambassador() && o!().global_vars.curtime - *self.get_last_fire() > 1.0)
    }
    pub fn get_info(&self) -> &WeaponInfo {
        let name = (o!().util.get_weapon_alias)(vmt_call!(self, get_weapon_id));
        let handle = (o!().util.get_weapon_info_handle)(name);
        (o!().util.get_weapon_info)(handle)
    }
    pub fn get_mode(&self) -> usize {
        let netvar = self.get_netvar(["m_iReloadMode"]).unwrap();
        (unsafe {
            transmute::<_, *const i32>((self as *const _ as *const u8).byte_add(netvar.offset - 8))
                .read()
        }) as usize
    }
}

pub type WeaponInfoHandle = i32;

impl Weapon {
    pub fn get_crit_bucket(&mut self) -> &mut f32 {
        unsafe {
            let netvar = self
                .get_netvar(["baseclass", "LocalWeaponData", "m_nViewModelIndex"])
                .unwrap();
            transmute(transmute::<_, *const f32>(self).byte_add(netvar.offset - 3 * 4))
        }
    }
    pub fn get_crit_checks(&mut self) -> &mut i32 {
        unsafe {
            let netvar = self
                .get_netvar(["baseclass", "LocalWeaponData", "m_nViewModelIndex"])
                .unwrap();
            transmute(transmute::<_, *const i32>(self).byte_add(netvar.offset - 2 * 4))
        }
    }
    pub fn get_crit_seed_requests(&mut self) -> &mut i32 {
        unsafe {
            let netvar = self
                .get_netvar(["baseclass", "LocalWeaponData", "m_nViewModelIndex"])
                .unwrap();
            transmute(transmute::<_, *const i32>(self).byte_add(netvar.offset - 4))
        }
    }
}
impl_has_vmt!(Weapon, VMTWeapon);

pub enum WeaponMode {
    Invalid = -1,
    Hitscan = 0,
    Projectile,
    Melee,
    Pda,
    Medigun,
    Consumable,
    Throwable,
}

//CTFWeaponBase{
//CTFWeaponBase m_bDisguiseWeapon
//CTFWeaponBase m_flEnergy
//CTFWeaponBase m_hExtraWearableViewModel
//CTFWeaponBase m_nKillComboCount
//CTFWeaponBase m_bLowered
//CTFWeaponBase m_nInspectStage
//CTFWeaponBase m_iReloadMode
//CTFWeaponBase baseclass LocalWeaponData m_iClip2
//CTFWeaponBase baseclass LocalWeaponData m_iSecondaryAmmoType
//CTFWeaponBase baseclass LocalWeaponData m_iClip1
//CTFWeaponBase baseclass LocalWeaponData m_nCustomViewmodelModelIndex
//CTFWeaponBase baseclass LocalWeaponData m_nViewModelIndex
//CTFWeaponBase baseclass LocalWeaponData m_iPrimaryAmmoType
//CTFWeaponBase baseclass LocalWeaponData m_bFlipViewModel
//CTFWeaponBase baseclass m_iViewModelIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_ProviderType
//CTFWeaponBase baseclass baseclass m_AttributeManager m_hOuter
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 004 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 004 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 004 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 004 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 009 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 009 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 009 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 009 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 019 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 019 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 019 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 019 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 015 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 015 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 015 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 015 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 008 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 008 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 008 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 008 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 001 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 001 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 001 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 001 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes lengthproxy lengthprop20
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 002 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 002 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 002 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 002 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 016 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 016 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 016 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 016 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 018 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 018 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 018 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 018 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 005 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 005 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 005 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 005 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 012 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 012 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 012 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 012 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 003 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 003 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 003 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 003 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 013 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 013 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 013 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 013 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 014 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 014 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 014 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 014 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 017 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 017 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 017 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 017 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 006 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 006 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 006 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 006 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 010 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 010 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 010 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 010 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 011 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 011 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 011 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 011 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 000 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 000 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 000 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 000 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 007 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 007 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 007 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_NetworkedDynamicAttributesForDemos m_Attributes 007 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_iAccountID
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_iEntityLevel
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_iItemIDHigh
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_iEntityQuality
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_iTeamNumber
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_iItemDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_iItemIDLow
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 019 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 019 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 019 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 019 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 005 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 005 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 005 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 005 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 000 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 000 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 000 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 000 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 003 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 003 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 003 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 003 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 002 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 002 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 002 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 002 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes lengthproxy lengthprop20
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 009 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 009 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 009 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 009 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 012 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 012 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 012 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 012 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 013 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 013 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 013 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 013 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 015 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 015 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 015 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 015 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 010 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 010 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 010 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 010 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 016 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 016 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 016 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 016 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 017 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 017 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 017 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 017 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 001 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 001 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 001 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 001 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 011 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 011 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 011 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 011 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 018 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 018 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 018 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 018 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 014 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 014 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 014 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 014 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 004 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 004 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 004 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 004 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 008 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 008 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 008 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 008 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 006 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 006 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 006 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 006 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 007 m_nRefundableCurrency
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 007 m_iAttributeDefinitionIndex
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 007 m_flValue
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_AttributeList m_Attributes 007 m_iRawValue32
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_bOnlyIterateItemViewAttributes
//CTFWeaponBase baseclass baseclass m_AttributeManager m_Item m_bInitialized
//CTFWeaponBase baseclass baseclass m_AttributeManager m_iReapplyProvisionParity
//CTFWeaponBase baseclass baseclass baseclass m_flFadeScale
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 014
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 016
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 022
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 007
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 002
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 010
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 003
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 013
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 020
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 021
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 019
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 011
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 023
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 004
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 017
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 009
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 005
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 006
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 012
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 008
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 001
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 000
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 015
//CTFWeaponBase baseclass baseclass baseclass m_flPoseParameter 018
//CTFWeaponBase baseclass baseclass baseclass m_fadeMaxDist
//CTFWeaponBase baseclass baseclass baseclass m_flModelWidthScale
//CTFWeaponBase baseclass baseclass baseclass m_nForceBone
//CTFWeaponBase baseclass baseclass baseclass m_nNewSequenceParity
//CTFWeaponBase baseclass baseclass baseclass m_vecForce
//CTFWeaponBase baseclass baseclass baseclass m_nMuzzleFlashParity
//CTFWeaponBase baseclass baseclass baseclass m_flEncodedController 001
//CTFWeaponBase baseclass baseclass baseclass m_flEncodedController 003
//CTFWeaponBase baseclass baseclass baseclass m_flEncodedController 002
//CTFWeaponBase baseclass baseclass baseclass m_flEncodedController 000
//CTFWeaponBase baseclass baseclass baseclass baseclass m_flElasticity
//CTFWeaponBase baseclass baseclass baseclass baseclass m_CollisionGroup
//CTFWeaponBase baseclass baseclass baseclass baseclass m_nModelIndexOverrides 003
//CTFWeaponBase baseclass baseclass baseclass baseclass m_nModelIndexOverrides 000
//CTFWeaponBase baseclass baseclass baseclass baseclass m_nModelIndexOverrides 001
//CTFWeaponBase baseclass baseclass baseclass baseclass m_nModelIndexOverrides 002
//CTFWeaponBase baseclass baseclass baseclass baseclass m_vecOrigin
//CTFWeaponBase baseclass baseclass baseclass baseclass m_bAnimatedEveryTick
//CTFWeaponBase baseclass baseclass baseclass baseclass m_angRotation
//CTFWeaponBase baseclass baseclass baseclass baseclass m_hOwnerEntity
//CTFWeaponBase baseclass baseclass baseclass baseclass m_flSimulationTime
//CTFWeaponBase baseclass baseclass baseclass baseclass m_fEffects
//CTFWeaponBase baseclass baseclass baseclass baseclass m_bSimulatedEveryTick
//CTFWeaponBase baseclass baseclass baseclass baseclass m_bAlternateSorting
//CTFWeaponBase baseclass baseclass baseclass baseclass m_iTextureFrameIndex
//CTFWeaponBase baseclass baseclass baseclass baseclass m_hEffectEntity
//CTFWeaponBase baseclass baseclass baseclass baseclass AnimTimeMustBeFirst m_flAnimTime
//CTFWeaponBase baseclass baseclass baseclass baseclass m_clrRender
//CTFWeaponBase baseclass baseclass baseclass baseclass m_iParentAttachment
//CTFWeaponBase baseclass baseclass baseclass baseclass m_iTeamNum
//CTFWeaponBase baseclass baseclass baseclass baseclass m_nModelIndex
//CTFWeaponBase baseclass baseclass baseclass baseclass m_nRenderFX
//CTFWeaponBase baseclass baseclass baseclass baseclass movetype
//CTFWeaponBase baseclass baseclass baseclass baseclass movecollide
//CTFWeaponBase baseclass baseclass baseclass baseclass m_ubInterpolationFrame
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_bUniformTriggerBloat
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_vecMinsPreScaled
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_nSurroundType
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMinsPreScaled
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_vecMaxs
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_vecMins
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMaxsPreScaled
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMins
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_usSolidFlags
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_vecSpecifiedSurroundingMaxs
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_vecMaxsPreScaled
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_nSolidType
//CTFWeaponBase baseclass baseclass baseclass baseclass m_Collision m_triggerBloat
//CTFWeaponBase baseclass baseclass baseclass baseclass m_flShadowCastDistance
//CTFWeaponBase baseclass baseclass baseclass baseclass m_nRenderMode
//CTFWeaponBase baseclass baseclass baseclass baseclass predictable_id m_bIsPlayerSimulated
//CTFWeaponBase baseclass baseclass baseclass baseclass predictable_id m_PredictableID
//CTFWeaponBase baseclass baseclass baseclass baseclass moveparent
//CTFWeaponBase baseclass baseclass baseclass m_nResetEventsParity
//CTFWeaponBase baseclass baseclass baseclass m_bClientSideAnimation
//CTFWeaponBase baseclass baseclass baseclass m_hLightingOriginRelative
//CTFWeaponBase baseclass baseclass baseclass m_nBody
//CTFWeaponBase baseclass baseclass baseclass m_bClientSideFrameReset
//CTFWeaponBase baseclass baseclass baseclass m_nSkin
//CTFWeaponBase baseclass baseclass baseclass m_fadeMinDist
//CTFWeaponBase baseclass baseclass baseclass m_flPlaybackRate
//CTFWeaponBase baseclass baseclass baseclass m_nHitboxSet
//CTFWeaponBase baseclass baseclass baseclass m_nSequence
//CTFWeaponBase baseclass baseclass baseclass m_flModelScale
//CTFWeaponBase baseclass baseclass baseclass serveranimdata m_flCycle
//CTFWeaponBase baseclass baseclass baseclass m_hLightingOrigin
//CTFWeaponBase baseclass baseclass m_bValidatedAttachedEntity
//CTFWeaponBase baseclass m_hOwner
//CTFWeaponBase baseclass m_iWorldModelIndex
//CTFWeaponBase baseclass LocalActiveWeaponData m_nNextThinkTick
//CTFWeaponBase baseclass LocalActiveWeaponData m_flNextSecondaryAttack
//CTFWeaponBase baseclass LocalActiveWeaponData m_flTimeWeaponIdle
//CTFWeaponBase baseclass LocalActiveWeaponData m_flNextPrimaryAttack
//CTFWeaponBase baseclass m_iState
//CTFWeaponBase LocalActiveTFWeaponData m_flLastFireTime
//CTFWeaponBase LocalActiveTFWeaponData m_flReloadPriorNextFire
//CTFWeaponBase LocalActiveTFWeaponData m_flEffectBarRegenTime
//CTFWeaponBase LocalActiveTFWeaponData m_flObservedCritChance
//CTFWeaponBase LocalActiveTFWeaponData m_flLastCritCheckTime
//CTFWeaponBase m_bResetParity
//CTFWeaponBase m_hExtraWearable
//CTFWeaponBase m_flInspectAnimEndTime
//CTFWeaponBase m_nKillComboClass
//CTFWeaponBase m_bBeingRepurposedForTaunt
//CTFWeaponBase m_bReloadedThroughAnimEvent
//CTFWeaponBase m_iConsecutiveShots
//}
