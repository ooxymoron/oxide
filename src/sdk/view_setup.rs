
use crate::math::{angles::Angles, vector::Vector3};

use super::VMatrix;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ViewSetup {
    pub x: isize,
    pub unscaled_x: isize,
    pub y: isize,
    pub unscaled_y: isize,
    pub width: isize,
    pub unscaled_width: isize,
    pub height: isize,
    pub stereo_eye: isize,
    pub unscaled_height: isize,
    pub ortho: bool,
    pub ortho_left: f32,
    pub ortho_top: f32,
    pub ortho_right: f32,
    pub ortho_bottom: f32,
    pub fov: f32,
    pub fov_viewmodel: f32,
    pub origin: Vector3,
    pub angles: Angles,
    pub z_near: f32,
    pub z_far: f32,
    pub z_near_viewmodel: f32,
    pub z_far_viewmodel: f32,
    pub render_to_subrect_if_larger_screen: bool,
    pub aspect_ratio: f32,
    pub off_center: bool,
    pub off_center_top: f32,
    pub off_center_bottom: f32,
    pub off_center_left: f32,
    pub off_center_right: f32,
    pub do_bloom_and_tone_mapping: bool,
    pub cache_full_scene_state: bool,
    pub view_to_projection_override: bool,
    pub view_to_projection: VMatrix,
}
