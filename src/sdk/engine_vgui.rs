use crate::cfn;

use super::WithVmt;



pub type EngineVgui = WithVmt<VMTEngineVgui>;

#[repr(C)]
#[derive(Debug,Clone)]
pub struct VMTEngineVgui {
    _pad1: [u8;4*15],
    pub paint:   cfn!((), &EngineVgui,isize),
}
