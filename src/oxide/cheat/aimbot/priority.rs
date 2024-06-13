use std::cmp::Ordering;

use crate::{
    error::OxideResult,
    math::{rtd, vector3::Vector3},
    sdk::entity::{player::Player, Entity},
    setting, vmt_call,
};

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
            return Some(Ordering::Greater);
        } else if self.ent < other.ent {
            return Some(Ordering::Less);
        }
        if self.hitbox > other.hitbox {
            return Some(Ordering::Greater);
        } else if self.hitbox < other.hitbox {
            return Some(Ordering::Less);
        }
        if self.point > other.point {
            return Some(Ordering::Greater);
        } else if self.point < other.point {
            return Some(Ordering::Less);
        }
        return Some(Ordering::Equal);
    }
}

impl Aimbot {
    pub fn point_priority(&self, target_point: Vector3) -> Option<isize> {
        let p_local = Player::get_local().unwrap();
        let my_eyes = vmt_call!(p_local.as_ent(), eye_position);

        let diff = target_point - my_eyes;
        let angle = diff.angle();
        if angle.pitch.abs() > 89.0 {
            return None;
        }
        let my_angle = vmt_call!(p_local.as_ent(), get_abs_angles);

        let target_forward = angle.to_vectors().forward;
        let my_forward = my_angle.to_vectors().forward;
        let angle = rtd((target_forward.dot(&my_forward)
            / (target_forward.len() * my_forward.len()))
        .acos());

        if angle > setting!(aimbot, fov) {
            return None;
        }

        Some(-angle as isize)
    }
    pub fn ent_priority(&self, ent: &mut Entity) -> OxideResult<Option<isize>> {
        Ok(ent.priority())
    }
}
