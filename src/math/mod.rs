use std::f32::consts::PI;

use self::vector::Vector3;

pub mod angles;
pub mod vector;

pub fn dtr(deg: f32) -> f32 {
    deg / 180f32 * PI
}

pub fn get_corners(
    pos: &Vector3,
    rotation: &[Vector3; 3],
    min: &Vector3,
    max: &Vector3,
) -> [Vector3; 8] {
    let mut corners = [
        Vector3::zeroed(),
        Vector3::zeroed(),
        Vector3::zeroed(),
        Vector3::zeroed(),
        Vector3::zeroed(),
        Vector3::zeroed(),
        Vector3::zeroed(),
        Vector3::zeroed(),
    ];
    for i in 0..8 {
        let x = if i & 0x1 != 0 { max.x } else { min.x };
        let y = if i & 0x2 != 0 { max.y } else { min.y };
        let z = if i & 0x4 != 0 { max.z } else { min.z };

        let corner = Vector3::new(x, y, z);

        let corner = corner.rotate(&rotation);
        corners[i] = corner + pos.clone()
    }
    corners
}
