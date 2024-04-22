use crate::math::vector::Vector3;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CameraThirdData {
    pitch: f32,
    yaw: f32,
    dist: f32,
    lag: f32,
    hull_min: Vector3,
    hull_max: Vector3,
}
