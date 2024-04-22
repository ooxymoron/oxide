use std::{mem::transmute, ptr::copy_nonoverlapping};

use libc::{PROT_EXEC, PROT_READ, PROT_WRITE};

use super::Hook;

#[derive(Debug)]
pub struct DetourHook {
    orig_bytes: [u8; 5],
    pub target: *mut u8,
    detour_jump: [u8; 5],
}

impl DetourHook {
    pub fn hook(target: *mut u8, detour_ptr: *const u8) -> DetourHook {
        unsafe {
            let orig_bytes: [u8; 5] = [0, 0, 0, 0, 0];
            copy_nonoverlapping(target, transmute(&orig_bytes), 5);

            let relative_detour_ptr = (detour_ptr as isize - (target as isize + 5)).to_be_bytes();
            let detour_jump: [u8; 5] = [
                0xe9,
                relative_detour_ptr[3],
                relative_detour_ptr[2],
                relative_detour_ptr[1],
                relative_detour_ptr[0],
            ];

            let start = target as usize & !0xFFF;
            let end = (target as usize + 5) & !0xFFF;

            libc::mprotect(
                transmute(start),
                transmute(end - start + 0xFFF),
                PROT_WRITE | PROT_READ | PROT_EXEC,
            );
            copy_nonoverlapping(transmute(&detour_jump), target, 5);

            DetourHook {
                orig_bytes,
                target,
                detour_jump,
            }
        }
    }
    pub fn patch(&mut self) {
        unsafe{
            copy_nonoverlapping(transmute(&self.detour_jump), self.target, 5);
        }
    }
    pub fn unpatch(&mut self) {
        unsafe{
            copy_nonoverlapping(transmute(&self.orig_bytes), self.target, 5);
        }
    }
}
impl Hook for DetourHook {
    fn restore(&mut self) {
        unsafe { copy_nonoverlapping(transmute(&self.orig_bytes), self.target, 5) }
    }
}

#[macro_export]
macro_rules! call_original {
    ($hookName:expr) => {
        let hook = o!().hooks.detour_hooks.get($hookName);
    };
}
