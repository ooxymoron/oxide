use std::{ffi::c_char, mem::transmute};

use libc::dlsym;
use std::ffi::CString;

use crate::{
    sdk::{
        entity::weapon::{ids::WeaponType, info::WeaponInfo, WeaponInfoHandle},
        interfaces::entity::Entity,
    },
    util::{
        get_handle,
        handles::{CLIENT, TIER0, VSTDLIB},
        sigscanner::find_sig,
    },
};

#[derive(Debug)]
pub struct Util {
    pub plat_float_time: extern "C" fn() -> f64,
    pub get_float_attribute: extern "C" fn(f32, *const c_char, &Entity, *const u8, bool) -> f32,
    pub get_weapon_alias: extern "C" fn(WeaponType) -> *const c_char,
    pub get_weapon_info_handle: extern "C" fn(*const c_char) -> WeaponInfoHandle,
    pub get_weapon_info: extern "C" fn(WeaponInfoHandle) -> &'static WeaponInfo,
    pub random_seed: extern "C" fn(i32),
    pub random_float: extern "C" fn(f32,f32) -> f32,
}

//get weapon alias
//83 FF 6D 77
//lookup weapon handle
////get weapon info from handle
//
impl Util {
    pub fn init() -> Util {
        let plat_float_time = CString::new("Plat_FloatTime").unwrap();
        let random_seed = CString::new("RandomSeed").unwrap();
        let random_float = CString::new("CGaussianRandomStream::RandomFloat(float, float)").unwrap();

        //
        Util {
            plat_float_time: unsafe {
                transmute(dlsym(
                    get_handle(TIER0).unwrap(),
                    plat_float_time.as_ptr(),
                ))
            },
            get_float_attribute: unsafe {
                transmute(
                    find_sig::<u8>(
                        CLIENT,
                        "55 31 C0 48 89 E5 41 57 41 56 41 55 49 89 F5 41 54 49 89 FC 53 89 CB",
                    )
                    .unwrap(),
                )
            },
            get_weapon_info: unsafe {
                transmute(find_sig::<u8>(CLIENT, "66 3B 3D ? ? ? ? 48 8D 05").unwrap())
            },
            get_weapon_info_handle: unsafe {
                transmute(
                    find_sig::<u8>(
                        CLIENT,
                        "48 85 FF 74 ? 55 48 89 E5 48 83 EC 10 48 8D 75 ? 48 89 7D ? 48 8D 3D ? ? ? ? E8 ? ? ? ? C9 C3 ? ? ? ? ? B8 FF FF FF FF C3 ? ? 55 48 89 e5",
                    )
                    .unwrap(),
                )
            },
            get_weapon_alias: unsafe { transmute(find_sig::<u8>(CLIENT, "83 FF 6D 77").unwrap()) },
            random_seed: unsafe {
                transmute(dlsym(
                    get_handle(VSTDLIB).unwrap(),
                    random_seed.as_ptr(),
                ))
            },
            random_float: unsafe {
                transmute(dlsym(
                    get_handle(VSTDLIB).unwrap(),
                    random_float.as_ptr(),
                ))
            },
        }
    }
}
