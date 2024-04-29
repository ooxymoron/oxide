//VDebugOverlay003

use crate::{
    cfn, hex_to_rgb,
    math::{angles::Angles, vector::Vector3},
    vmt_call,
};

use super::WithVmt;

pub type DebugOverlay = WithVmt<VMTDebugOverlay>;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTDebugOverlay {
    _pad: [usize; 1],
    pub add_box_overlay: cfn!(
        (),
        &DebugOverlay,
        &Vector3,
        &Vector3,
        &Vector3,
        &Angles,
        u8,
        u8,
        u8,
        u8,
        f32
    ),
}
impl DebugOverlay {
    pub fn add_box_overlay(
        &self,
        pos: &Vector3,
        size: f32,
        color: usize,
        alpha: u8,
        duration: f32,
    ) {
        let (r, g, b) = hex_to_rgb!(color);
        let size = size / 2.0;
        vmt_call!(
            self,
            add_box_overlay,
            pos,
            &Vector3::new(-size, -size, -size),
            &Vector3::new(size, size, size),
            &Angles::new(0.0, 0.0, 0.0),
            r,
            g,
            b,
            alpha,
            duration
        );
    }
}
