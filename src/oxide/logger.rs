use crate::{interface, o};

#[derive(Debug)]
pub struct Logger {}

impl Logger {
    pub fn log(&self, text: &str) {
        let text = format!("[OXIDE]: {}\n", text);
        interface!(cvar).console_print(&text);
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        {
            let text = format!($($arg)*);

           #[allow(unused_unsafe)]
            unsafe{
                    eprint!("{}\n", text);
                    if let Some(oxide) = crate::OXIDE{
                        let o  = &mut *(oxide as *mut _ as *mut crate::Oxide);
                        o.logger.log(&text);

                    }
            }
        }
    };
}
