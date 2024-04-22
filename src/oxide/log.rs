use std::{ffi::{CString, VaList}, intrinsics::{breakpoint, transmute_unchecked}, mem::transmute};

use sdl2_sys::va_list;

use crate::{interface, o, sdk::cvar::Color, util::debug::print_module_addres_offset, vmt_call};

#[derive(Debug)]
pub struct Logger {}

impl Logger {
    pub fn log(&self, text: &str) {
        let text = CString::new("test").unwrap();
        let color = Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        dbg!(&text);
        vmt_call!(interface!(cvar),color_console_print,&color,text.as_ptr(),"ss");
        //interface!(cvar).console_print(&color, text.as_ptr());
    }
}
