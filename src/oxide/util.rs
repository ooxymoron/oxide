use std::{ffi::c_char, mem::transmute};

use libc::dlsym;
use std::ffi::CString;

use crate::{
    sdk::{
        entity::{
            player::Player,
            weapon::{ids::WeaponId, info::WeaponInfo, Weapon, WeaponInfoHandle},
        },
        interfaces::entity::Entity, user_cmd::UserCmd,
    },
    util::{
        get_handle,
        handles::{CLIENT, SERVER, TIER0, VSTDLIB},
        sigscanner::find_sig,
    },
};

#[derive(Debug)]
pub struct Util {
    pub plat_float_time: extern "C" fn() -> f64,
    pub get_float_attribute: extern "C" fn(f32, *const c_char, &Entity, *const u8, bool) -> f32,
    pub get_weapon_alias: extern "C" fn(WeaponId) -> *const c_char,
    pub get_weapon_info_handle: extern "C" fn(*const c_char) -> WeaponInfoHandle,
    pub get_weapon_info: extern "C" fn(WeaponInfoHandle) -> &'static WeaponInfo,
    pub random_seed: extern "C" fn(i32),
    pub random_float: extern "C" fn(f32, f32) -> f32,
    pub random_int: extern "C" fn(i32, i32) -> i32,
    pub is_crit_boosted: extern "C" fn(&Player) -> bool,
    pub md5_pseudorandom: extern "C" fn (i32) -> i32,
    pub add_to_crit_bucket: extern "C" fn (f32,&Weapon),
    pub check_if_can_withdraw_form_critbucket: extern "C" fn (f32,&Weapon),
    pub set_prediction_seed: extern "C" fn (&UserCmd),
    pub apply_fire_delay: extern "C" fn (f32,&Weapon) -> f32,
}

impl Util {
    pub fn init() -> Util {
        let plat_float_time = CString::new("Plat_FloatTime").unwrap();
        let random_seed = CString::new("RandomSeed").unwrap();
        let random_float = CString::new("RandomFloat").unwrap();
        let random_int = CString::new("RandomInt").unwrap();

        unsafe {
            Util {
                plat_float_time: transmute(dlsym(
                    get_handle(TIER0).unwrap(),
                    plat_float_time.as_ptr(),
                )),
                get_float_attribute: 
                    transmute(
                        find_sig::<u8>(
                            CLIENT,
                            "55 31 C0 48 89 E5 41 57 41 56 41 55 49 89 F5 41 54 49 89 FC 53 89 CB",
                        )
                        .unwrap(),
                    )
                ,
                get_weapon_info: 
                    transmute(find_sig::<u8>(CLIENT, "66 3B 3D ? ? ? ? 48 8D 05").unwrap())
                ,
                get_weapon_info_handle: 
                    transmute(
                    find_sig::<u8>(
                        CLIENT,
                        "48 85 FF 74 ? 55 48 89 E5 48 83 EC 10 48 8D 75 ? 48 89 7D ? 48 8D 3D ? ? ? ? E8 ? ? ? ? C9 C3 ? ? ? ? ? B8 FF FF FF FF C3 ? ? 55 48 89 e5",
                    )
                    .unwrap(),
                )
                ,
                get_weapon_alias: 
                    transmute(find_sig::<u8>(CLIENT, "83 FF 6D 77").unwrap())
                ,
                is_crit_boosted: 
                    transmute(find_sig::<u8>(CLIENT, "55 BE 0B 00 00 00 48 89 E5").unwrap())
                ,
                md5_pseudorandom: 
                    transmute(find_sig::<u8>(SERVER, "55 31 C0 B9 09 00 00 00 48 89 E5 41 54").unwrap())
                ,

                random_seed: 
                    transmute(dlsym(get_handle(VSTDLIB).unwrap(), random_seed.as_ptr()))
                ,
                random_float: 
                    transmute(dlsym(get_handle(VSTDLIB).unwrap(), random_float.as_ptr()))
                ,
                random_int: 
                    transmute(dlsym(get_handle(VSTDLIB).unwrap(), random_int.as_ptr()))
                ,
                add_to_crit_bucket: 
                    transmute(find_sig::<u8>(CLIENT, "48 8B 05 ? ? ? ? 0F 28 C8 F3 0F 10 87").unwrap())
                ,
                check_if_can_withdraw_form_critbucket: 
                    transmute(find_sig::<u8>(CLIENT, "55 48 89 E5 53 48 89 FB 48 83 EC 18 F3 0F 11 45 ? 83 87 ? ? ? ? 01").unwrap())
                ,
                set_prediction_seed: 
                    transmute(find_sig::<u8>(CLIENT, "48 85 FF 74 ? 8B 57 ? 48 8D 05").unwrap())
                ,
                apply_fire_delay: 
                    transmute(find_sig::<u8>(CLIENT, "55 31 D2 48 89 FE B9 01 00 00 00 48 89 E5 41 56").unwrap())
                ,
            }
        }
    }
}

//is_crit_boosted
//
