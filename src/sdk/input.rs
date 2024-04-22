use crate::cfn;

use super::WithVmt;



pub type Input = WithVmt<VMTInput>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTInput {
    _pad: [usize; 18],
    pub activate_mouse: cfn!((), &Input),
    pub deactivate_mouse: cfn!((), &Input),
}
