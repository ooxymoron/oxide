use self::view_setup::ViewSetup;

use super::*;

pub type RenderView = WithVmt<VMTRenderView>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FloatRGBA(f32, f32, f32, f32);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTRenderView {
    _pad1: [u32; 4],
    pub set_blend: cfn!((), &'static RenderView, f32),
    pub get_blend: cfn!(f32, &'static RenderView),
    pub set_color_modulation: cfn!((), &'static RenderView, &'static FloatRGBA),
    pub get_color_modulation: cfn!((), &'static RenderView, &'static mut FloatRGBA),
    _pad2: [u32; 42],
    pub get_matrices_for_view: cfn!(
        (),
        &RenderView,
        &ViewSetup,
        &VMatrix,
        &VMatrix,
        &VMatrix,
        &VMatrix
    ),
}
