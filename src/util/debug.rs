use std::{intrinsics::{breakpoint, transmute_unchecked}, mem::transmute};

use crate::{util::{get_handle, LinkMap}};

pub fn print_bytes<const T:usize,U>(addr:*const U) {
    unsafe{
        let bytes = transmute_unchecked::<_,*const [u8;T]>(addr).read_unaligned();
        for byte in bytes {
            eprint!("{:#04X} ",byte);
        }
        eprint!("\n");
    }

}
pub fn print_module_addres_offset(addr: *const u8) {
    //TODO: uaot detect which module were in
    let link_map: *const LinkMap = unsafe { transmute(get_handle("./tf/bin/linux64/client.so").unwrap()) };
    let start = unsafe{link_map.read().addr};
    eprintln!("{:#0X} ", addr as i64 - start as i64 + 0x100000);


}
