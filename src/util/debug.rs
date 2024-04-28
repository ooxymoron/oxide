use std::{intrinsics::transmute_unchecked, mem::transmute};

use crate::{
    log,
    util::{get_handle, LinkMap},
};

pub fn print_bytes<const T: usize, U>(addr: *const U) {
    unsafe {
        let bytes = transmute_unchecked::<_, *const [u8; T]>(addr).read_unaligned();
        let mut buffer = String::new();
        for byte in bytes {
            buffer += &format!("{:#04X} ", byte);
        }
        log!("{}\n", buffer);
    }
}
pub fn print_module_addres_offset(addr: *const u8, module: &str) {
    //TODO: uaot detect which module were in
    let link_map: *const LinkMap =
        unsafe { transmute(get_handle(module).unwrap()) };
    let start = unsafe { link_map.read().addr };
    log!("{:#0X} ", addr as i64 - start as i64 + 0x100000);
}
