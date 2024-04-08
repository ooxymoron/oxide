use std::mem::transmute;

use super::{get_handle, LinkMap};

fn parse_sig_str(sig: &str) -> Vec<Option<u8>> {
    let split: Vec<&str> = sig.split(" ").collect();
    split
        .into_iter()
        .map(|x| {
            if x == "??" || x == "?" {
                None
            } else {
                Some(u8::from_str_radix(x, 16).unwrap())
            }
        })
        .collect()
}

pub fn find_sig(module: &str, sig: &str) -> *const u8 {
    unsafe {
        let link_map: *const LinkMap = transmute(get_handle(module).unwrap());

        let mod_size = link_map.read().phdr.read().p_memsz;
        let base_addr = link_map.read().addr as *const u8;

        let parsed = parse_sig_str(sig);

        let sig_bytes = parsed.as_slice();
        let sig_len = sig_bytes.len();

        let mut sig_index = 0;
        for i in 0..mod_size {
            let ptr = (base_addr as u64 + i) as *const _;

            let sig_byte = sig_bytes[sig_index];
            let matches = match sig_byte {
                Some(s) => s == *ptr,
                None => true,
            };

            if matches {
                sig_index += 1;
                if sig_index != sig_len {
                    continue;
                }

                let start = ptr.sub(sig_index - 1);
                return if *start == 0xE8 {
                    // relative call
                    let asm_ptr = *(start.add(1) as *const u32);
                    start.sub(!asm_ptr as usize).add(4)
                } else {
                    start
                };
            } else if sig_index > 0 {
                sig_index = 0;
            }
        }

        return std::ptr::null();
    }
}
