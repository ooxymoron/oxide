use crate::cfn;

use super::WithVmt;



pub type EngineVgui = WithVmt<VMTEngineVgui>;

#[repr(C)]
#[derive(Debug,Clone)]
pub struct VMTEngineVgui {
    _pad1: [isize;15],
    pub paint:   cfn!((), &EngineVgui,isize),
}
