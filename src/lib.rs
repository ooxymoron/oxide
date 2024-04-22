#![feature(
    c_variadic,
    pointer_is_aligned,
    associated_type_defaults,
    stmt_expr_attributes,
    core_intrinsics,
    unboxed_closures,
    inherent_associated_types,
    absolute_path
)]
#![allow(improper_ctypes_definitions, internal_features, incomplete_features)]

use std::{
    alloc::{alloc, Layout},
    error::Error,
    ffi::*,
    mem::ManuallyDrop,
    thread,
};

use crate::{draw::Draw, oxide::Oxide, settings::Settings};

pub mod draw;
pub mod error;
pub mod math;
pub mod oxide;
pub mod sdk;
pub mod settings;
pub mod util;
pub mod netvars;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

static mut OXIDE: Option<*mut c_void> = None;
static mut DRAW: Option<*mut c_void> = None;
static mut SETTINGS: Option<*mut c_void> = None;


unsafe fn main() -> Result<(), std::boxed::Box<dyn Error>> {
    println!("loading");

    init_global!(SETTINGS,Settings::load()?,Settings);
    init_global!(OXIDE,Oxide::init()?,Oxide);
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");
    o!().logger.log("test");

    println!("loaded");
    Ok(())
}

#[link_section = ".init_array"]
#[allow(unused)]
static LOAD: unsafe extern "C" fn() = {
    #[link_section = ".text.startup"]
    unsafe extern "C" fn load() {
        libc::atexit(unload);

        thread::spawn(|| unsafe {
            if let Err(e) = main() {
                eprintln!("{}", e);
            }
        });
    }
    load
};

#[allow(unused)]
#[link_section = ".text.exit"]
extern "C" fn unload() {
    unsafe {

        println!("unloading");

        if DRAW.is_some() {
            d!().restore();
            std::ptr::drop_in_place(d!());
        }
        if OXIDE.is_some() {
            o!().restore();
            std::ptr::drop_in_place(o!());
        }

        println!("unloaded");
    }
}
