use crate::{vmt_call, error::OxideResult, math::vector::Vector3, sdk::entity::Entity, setting};

use super::Aimbot;

impl Aimbot {
    pub fn point_priority(&self, target_point: Vector3) -> Option<isize> {
        let p_local = Entity::get_local().unwrap();
        let my_eyes = vmt_call!(p_local.as_ent(), eye_position);

        let diff = my_eyes - target_point;
        let angle = diff.angle();
        let my_angle = p_local.angle.clone();

        let distance_to_center = (((angle.yaw - my_angle.yaw)
            .min(360f32 - angle.yaw + my_angle.yaw)
            .abs()
            % 360f32)
            .powi(2)
            + (angle.pitch - my_angle.pitch).abs().powi(2))
        .sqrt();

        if distance_to_center > setting!(aimbot, fov) {
            return None;
        }

        Some(-distance_to_center as isize)
    }
    pub fn ent_priority(&self, ent: &mut Entity) -> OxideResult<Option<isize>> {
        let p_local = &*Entity::get_local().unwrap();
        if vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number) {
            return Ok(None);
        }
        return Ok(Some(1));
    }
}
