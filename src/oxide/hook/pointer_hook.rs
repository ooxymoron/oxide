#[macro_export]
macro_rules! define_hook{
    ($name:ident,$stringName:expr,$proxyFn:expr,$return:ty,$default:expr,$($argName:ident,$argType:ty),*) => {
        use crate::{cfn,o,OXIDE,oxide::hook::Hook,log};
        use core::intrinsics::{transmute,transmute_unchecked};
        use std::panic::{catch_unwind,AssertUnwindSafe};
        use libc::{PROT_EXEC, PROT_READ, PROT_WRITE};

        type RawHookFn = extern fn ($($argType),*) -> $return;
        type ProxyHookFn =  extern fn ($($argType),*,RawHookFn) -> $return;


        #[derive(Debug)]
        pub struct $name
        {
            pub org: RawHookFn,
            pub target: &'static mut RawHookFn,
        }

        impl $name {
            pub type RawFn = RawHookFn;
            pub type ProxyFn = ProxyHookFn;
            fn restore(&mut self) {
                *self.target = self.org
            }
            pub fn init(target: &RawHookFn) -> Self {
                let target = unsafe {transmute_unchecked::<_,&'static mut RawHookFn>(target)};
                let org = (*target).clone();
                let hook = $name { org, target};

                let page = hook.target as *const _ as usize & !0xFFF;

                unsafe{
                    libc::mprotect(
                        transmute(page),
                        transmute(page + 0xFFF),
                        PROT_WRITE | PROT_READ | PROT_EXEC,
                    );
                }
                *hook.target = $name::hook_fn;
                hook
            }
            pub fn name() -> String{
                $stringName.to_owned()
            }
            #[allow(unused)]
            extern "C" fn hook_fn($($argName:$argType),*) -> $return{
                unsafe{

                    if OXIDE.is_none() {
                        return $default;
                    }

                    let mut hook = o!().hooks.get::<Self>(Self::name());


                    match catch_unwind( AssertUnwindSafe(||($proxyFn)($($argName),*,hook.org))) {
                        Ok(res) => res,
                        Err(e) => {
                            log!("unhandled error in {} bofere hook: {:?}",$stringName,e);
                            $default
                        }
                    }

                }
            }
        }
        impl Hook for $name {
            fn restore(&mut self) {
                self.restore()
            }
        }
    }
}
