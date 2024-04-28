//VDebugOverlay003

use crate::cfn;

use super::WithVmt;

pub type DebugOverlay = WithVmt<VMTDebugOverlay>;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTDebugOverlay {
    _pad: [usize; 1],
    //pub temp: cfn!(*mut DebugOverlay ),
}
