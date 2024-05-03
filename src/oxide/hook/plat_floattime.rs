use crate::call_original;

pub const NAME: &str = "Plat_FloatTime";

pub type PlatFloatTime = extern "C" fn() -> f64;

pub extern "C" fn hook() -> f64 {
    let org = call_original!(NAME, PlatFloatTime) + 262144.0;
    org
}
