use std::mem::transmute;

use libc::dlsym;
use std::ffi::CString;

use crate::util::{get_handle, handles::TIER0};

#[derive(Debug)]
pub struct Util {
    pub plat_float_time: extern "C" fn() -> f64,
}
impl Util {
    pub fn init() -> Util {
        let name = CString::new("Plat_FloatTime").unwrap();
        Util {
            plat_float_time: unsafe {
                transmute::<_, extern "C" fn() -> f64>(dlsym(
                    get_handle(TIER0).unwrap(),
                    name.as_ptr(),
                ))
            },
        }
    }
}
