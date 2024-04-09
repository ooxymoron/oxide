pub mod hooks;

pub mod create_move;
pub mod frame_stage_notify;
pub mod level_shutdown;
pub mod override_view;
pub mod paint;
pub mod paint_traverse;
pub mod poll_event;
pub mod swap_window;
pub mod run_command;

pub trait Hook: std::fmt::Debug {
    fn restore(&mut self);
}

#[macro_export]
macro_rules! define_hook{
    ($name:ident,$stringName:expr,$return:ty,$default:expr,$subhooks:expr,$($argName:ident,$argType:ty),*) => {
        use crate::{cfn,o,OXIDE,oxide::hook::Hook,error::OxideResult};
        use core::intrinsics::{transmute_unchecked};
        use std::panic::{catch_unwind,AssertUnwindSafe};

        type RawHookFn = cfn!($return,$($argType),*);
        type BeforeHookFn =  fn ($($argType),*) -> OxideResult<bool>;
        type AfterHookFn = fn ($($argType),*,&mut $return) -> OxideResult<()>;


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
                //dbg!("b",$stringName);

                let mut hook = o!().hooks.get::<Self>(Self::name());

                let mut should_run = true;

                if let Some(fun) = &hook.before {
                    match catch_unwind( AssertUnwindSafe(||(fun)($($argName),*))) {
                        Ok(Ok(res)) => should_run = res,
                        Ok(Err(e)) => {
                            eprintln!("error in {} bofere hook: {}",$stringName,e);
                            return $default
                        }
                        Err(e) => {
                            eprintln!("unhandled error in {} bofere hook: {:?}",$stringName,e);
                            return $default
                        }
                    }

                }


                let mut return_value = if should_run {
                    (hook.org)($($argName),*)
                } else {
                    $default
                };

                if let Some(fun) = hook.after {
                    match catch_unwind( AssertUnwindSafe(||(fun)($($argName),*,&mut return_value))) {
                        Ok(Err(e)) => {
                            eprintln!("error in {} bofere hook: {}",$stringName,e);
                            return $default;
                        }
                        Err(e) => {
                            eprintln!("unhandled error in {} bofere hook: {:?}",$stringName,e);
                            return $default;
                        }
                        _ => {}
                    }
                }
                //dbg!("a",$stringName);
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
