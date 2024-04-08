
use super::*;

pub type Panel = WithVmt<VMTPanel>;


pub type VPanel = usize;


#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTPanel {
    _pad1: [u32; 37],
    pub get_name: cfn!(*const i8, *const Panel, VPanel),
    _pad2: [u32; 4],
    pub paint_traverse: cfn!((), &Panel, VPanel, bool, bool),
}
