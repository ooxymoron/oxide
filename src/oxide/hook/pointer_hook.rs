
#[macro_export]
macro_rules! define_hook{
    ($name:ident,$stringName:expr,$return:ty,$default:expr,$subhooks:expr,$($argName:ident,$argType:ty),*) => {
        use crate::{cfn,o,OXIDE,oxide::hook::Hook,log};
        use core::intrinsics::{transmute,transmute_unchecked};
        use std::panic::{catch_unwind,AssertUnwindSafe};
        use libc::{PROT_EXEC, PROT_READ, PROT_WRITE};

        type RawHookFn = cfn!($return,$($argType),*);
        type BeforeHookFn =  fn ($($argType),*) -> Option<$return>;
        type AfterHookFn = fn ($($argType),*,&mut $return);


        #[derive(Debug)]
        pub struct $name
        {
            pub org: RawHookFn,
            pub target: &'static mut RawHookFn,
            pub before: Option<BeforeHookFn>,
            pub after: Option<AfterHookFn>,
        }

        impl $name {
            pub type RawFn = RawHookFn;
            pub type BeforeFn = BeforeHookFn;
            pub type AfterFn = AfterHookFn;
            fn restore(&mut self) {
                *self.target = self.org
            }
            pub fn init(target: &RawHookFn) -> Self {
                let target = unsafe {transmute_unchecked::<_,&'static mut RawHookFn>(target)};
                let org = (*target).clone();
                let mut hook = $name { org, target, before: None, after: None};

                let page = hook.target as *const _ as usize & !0xFFF;

                unsafe{
                    libc::mprotect(
                        transmute(page),
                        transmute(page + 0xFFF),
                        PROT_WRITE | PROT_READ | PROT_EXEC,
                    );
                }
                *hook.target = $name::hook_fn;
                $subhooks(&mut hook);
                hook
            }
            pub fn name() -> String{
                $stringName.to_owned()
            }
            #[allow(unused)]
            unsafe extern "C-unwind" fn hook_fn($($argName:$argType),*) -> $return{

                if OXIDE.is_none() {
                    return $default;
                }

                let mut hook = o!().hooks.get::<Self>(Self::name());


                let mut custom_return_value = if let Some(fun) = &hook.before {
                    match catch_unwind( AssertUnwindSafe(||(fun)($($argName),*))) {
                        Ok(res) => res,
                        Err(e) => {
                            log!("unhandled error in {} bofere hook: {:?}",$stringName,e);
                            None
                        }
                    }

                } else { None };


                let mut return_value = if let Some(return_val) = custom_return_value {
                    return_val
                } else {
                    (hook.org)($($argName),*)
                };

                if let Some(fun) = hook.after {
                    match catch_unwind( AssertUnwindSafe(||(fun)($($argName),*,&mut return_value))) {
                        Err(e) => {
                            log!("unhandled error in {} bofere hook: {:?}",$stringName,e);
                            return $default;
                        }
                        _ => {}
                    }
                }
                return return_value;
            }
        }
        impl Hook for $name {
            fn restore(&mut self) {
                self.restore()
            }
        }
    }
}
