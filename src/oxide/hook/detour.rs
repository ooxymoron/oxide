use std::{mem::transmute, ptr::copy_nonoverlapping};

use libc::{PROT_EXEC, PROT_READ, PROT_WRITE};

const PATCH_SIZE: usize = 14;

use super::Hook;

#[derive(Debug)]
pub struct DetourHook {
    orig_bytes: [u8; PATCH_SIZE],
    pub target: *mut u8,
    detour_jump: [u8; PATCH_SIZE],
}

impl DetourHook {
    pub fn hook(target: *mut u8, detour_ptr: *const u8) -> DetourHook {
        unsafe {
            let orig_bytes = [0u8; PATCH_SIZE];
            copy_nonoverlapping(target, transmute(&orig_bytes), PATCH_SIZE);

            let detour_ptr = (detour_ptr as usize).to_le_bytes();
            let detour_jump: [u8; PATCH_SIZE] = [
                0xff,
                0x25,
                0x00,
                0x00,
                0x00,
                0x00,
                detour_ptr[0],
                detour_ptr[1],
                detour_ptr[2],
                detour_ptr[3],
                detour_ptr[4],
                detour_ptr[5],
                detour_ptr[6],
                detour_ptr[7],
            ];

            let start = target as usize & !0xFFF;
            let end = (target as usize + PATCH_SIZE) & !0xFFF;

            libc::mprotect(
                transmute(start),
                transmute(end - start + 0xFFF),
                PROT_WRITE | PROT_READ | PROT_EXEC,
            );
            copy_nonoverlapping(transmute(&detour_jump), target, PATCH_SIZE);

            DetourHook {
                orig_bytes,
                target,
                detour_jump,
            }
        }
    }
    pub fn patch(&mut self) {
        unsafe {
            copy_nonoverlapping(transmute(&self.detour_jump), self.target, PATCH_SIZE);
        }
    }
    pub fn unpatch(&mut self) {
        unsafe {
            copy_nonoverlapping(transmute(&self.orig_bytes), self.target, PATCH_SIZE);
        }
    }
}
impl Hook for DetourHook {
    fn restore(&mut self) {
        self.unpatch();
    }
}

#[macro_export]
macro_rules! call_original {
    ($name:expr,$t: ty $(,$args: expr)*) => {
        {
            use std::mem::transmute;
            let hook = crate::o!().hooks.detour_hooks.get_mut($name).unwrap();
            hook.unpatch();
            let res = unsafe {
                transmute::<_, $t>(hook.target)(
                    $($args),*
                )
            };
            hook.patch();
            res
        }
    };
}
