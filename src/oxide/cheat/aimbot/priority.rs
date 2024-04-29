use std::cmp::Ordering;

use crate::{error::OxideResult, math::vector::Vector3, sdk::entity::{player::Player, Entity}, setting, vmt_call};

use super::Aimbot;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Priority {
    pub ent: isize,
    pub hitbox: isize,
    pub point: isize,
}
impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.ent > other.ent {
            return Some(Ordering::Greater)
        } else if self.ent < other.ent {
            return Some(Ordering::Less)
        }
        if self.hitbox > other.hitbox {
            return Some(Ordering::Greater)
        } else if self.hitbox < other.hitbox {
            return Some(Ordering::Less)
        }
        if self.point > other.point {
            return Some(Ordering::Greater)
        } else if self.point < other.point {
            return Some(Ordering::Less)
        }
        return Some(Ordering::Equal);
    }
}

impl Aimbot {
    pub fn point_priority(&self, target_point: Vector3) -> Option<isize> {
        let p_local = Player::get_local().unwrap();
        let my_eyes = vmt_call!(p_local.as_ent(), eye_position);


        let diff = my_eyes - target_point;
        let angle = diff.angle();
        let my_angle = vmt_call!(p_local.as_ent(),get_abs_angles);

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
        let p_local = Player::get_local().unwrap();
        if vmt_call!(ent, get_team_number) == vmt_call!(p_local.as_ent(), get_team_number) {
            return Ok(None);
        }
        return Ok(Some(1));
    }
}
