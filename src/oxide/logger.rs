use crate::{interface, o};

#[derive(Debug)]
pub struct Logger {}

impl Logger {
    pub fn log(&self, text: &str) {
        let text = format!("[OXIDE]: {}\n", text);
        interface!(cvar).console_print(&text);
        eprint!("{}", text);
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        let text = format!($($arg)*);
        crate::o!().logger.log(&text);
    };
}
