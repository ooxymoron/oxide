use std::{ffi::CString, intrinsics::breakpoint};

use crate::{interface, o, sdk::cvar::Color, vmt_call};



#[derive(Debug)]
pub struct Logger {

}

impl Logger {
    pub fn log(&self,text: &str) {
        let text = CString::new("test").unwrap();
        let color = Color{r:255,g:0,b:0,a:255};
        dbg!(&text);
        vmt_call!(interface!(cvar),console_color_print,&color,text.as_ptr())
    }
}
