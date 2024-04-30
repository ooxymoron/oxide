#[macro_export]
macro_rules! cfn {
    ($r:ty,$($t:ty),*) => {
        extern fn($($t), *) -> $r
    }
}

#[macro_export]
macro_rules! init_global {
    ($dest: expr,$val: expr,$t:ty) => {
        let tmp = alloc(Layout::new::<$t>()) as *mut _ as *mut ManuallyDrop<$t>;
        *tmp = ManuallyDrop::new($val);
        $dest = Some(tmp as *mut _ as *mut c_void);
    }
}

#[macro_export]
macro_rules! o {
    () => {
        #[allow(unused_unsafe)]
        unsafe {
            use crate::{Oxide, OXIDE};
            &mut *(OXIDE.unwrap() as *mut _ as *mut Oxide)
        }
    };
}

#[macro_export]
macro_rules! d {
    () => {
        #[allow(unused_unsafe)]
        unsafe {
            use crate::{Draw, DRAW};
            &mut *(DRAW.unwrap() as *mut _ as *mut Draw)
        }
    };
}

#[macro_export]
macro_rules! setting {
    ($($path:ident), +) => {
        #[allow(unused_unsafe)]
        unsafe {
             
            *$crate::s!().$($path.)+lock().unwrap()
        }
    };
}
#[macro_export]
macro_rules! s {
    () => {
        #[allow(unused_unsafe)]
        unsafe {
            use crate::{Settings, SETTINGS};
            &mut *(SETTINGS.unwrap() as *mut _ as *mut Settings)
        }
    };
}

#[macro_export]
macro_rules! interface_vmt {
    ($n:ident) => {
        (*o!().interfaces.$n.get_vmt())
    };
}

#[macro_export]
macro_rules! interface {
    ($n:ident) => {{
        use crate::o;
        o!().interfaces.$n.interface_ref()
    }};
}
#[macro_export]
macro_rules! vmt_call {
    ($i:expr,$f:ident $(,$args: expr)*) => {
        #[allow(unused_unsafe)]
        unsafe{
            (($i.vmt.read_unaligned()).$f)($i,$($args),*)
        }
    };
}
#[macro_export]
macro_rules! call_interface {
    ($i:ident,$f:ident $(,$args: expr)*) => {
        ((*interface_ref!($i)).vmt.$f)(interface_ref!($i),$($args),*)
    };
}

#[macro_export]
macro_rules! impl_has_vmt {
    ($t:tt,$tv:tt) => {
        use crate::sdk::HasVmt;
        impl HasVmt<$tv> for $t {
            fn get_vmt(&self) -> &'static $tv {
                unsafe { &*self.vmt }
            }

            fn set_vmt(&mut self, vmt: *mut $tv) {
                self.vmt = vmt
            }
        }
    };
}

#[macro_export]
macro_rules! hex_to_rgb {
    ($h:expr) => {
        (($h >> 16) as u8, ($h >> 8) as u8, $h as u8)
    };
}
#[macro_export]
macro_rules! rgb_to_hex {
    ($r:expr,$g:expr, $b:expr) => {
        (($r as usize) << 16) + (($g as usize) << 8) + $b as usize
    };
}

#[macro_export]
macro_rules! define_offset {
    ($name: ident, $offset: expr, $type: ty) => {
    pub fn $name(&self) -> &mut $type {
        unsafe { transmute((self as *const _ as *const u8).byte_add($offset)) }
    }
        
    };
}

#[macro_export]
macro_rules! c_str_to_str {
    ($c_str: expr) => {
        unsafe{std::ffi::CStr::from_ptr($c_str).to_str().unwrap()}
        
    };
}

#[macro_export]
macro_rules! get_cheat {
    ($cheat: ident) => {
        crate::o!().cheats.get::<$cheat>($cheat::name())
    };
}
