use std::f32::consts::PI;

use crate::{
    error::OxideResult,
    math::{angles::Angles, dtr},
    sdk::{
        entity::{flags::Flag, player::player_class::PlayerClass, Entity, WaterLevel}, interfaces::cvar::get_cvar, user_cmd::{ButtonFlags, UserCmd}
    },
    setting,
};

use super::Cheat;

const SPEED_VAR: f32 = 6062.0;
const WISH_SPEED: f32 = 30.0;

#[derive(Debug)]
pub struct Movement {
    double_jumped: bool,
    jumped_last_cmd: bool,
}
impl Movement {
    pub fn init() -> Movement {
        Movement {
            double_jumped: false,
            jumped_last_cmd: false,
        }
    }
    pub fn name() -> &'static str {
        "Movement"
    }
    pub fn create_move(&mut self, cmd: &mut UserCmd) -> OxideResult<()> {
        let p_local = Entity::get_local()?;
        if p_local.get_flags().get(Flag::INWATER)
            || p_local.get_flags().get(Flag::SWIM)
            || *p_local.get_water_level() > WaterLevel::Feet
        {
            return Ok(());
        }
        let on_ground = p_local.get_flags().get(Flag::ONGROUND);
        let jumping = cmd.buttons.get(ButtonFlags::InJump);
        if on_ground {
            self.double_jumped = false
        } else if jumping
            && matches!(*p_local.get_player_class(), PlayerClass::Scout)
            && !self.double_jumped
            && !self.jumped_last_cmd
        {
            self.double_jumped = true;
        } else {
            self.bhop(cmd)?;
            self.auto_strafe(cmd)?;
        }
        self.jumped_last_cmd = jumping;

        Ok(())
    }
    pub fn correct_movement(&mut self, cmd: &mut UserCmd, org_cmd: &UserCmd) {
        if org_cmd.viewangles.yaw != cmd.viewangles.yaw {
            let (corrected_forward, correct_side) = self.calculate_correct_movement(
                org_cmd.viewangles,
                &cmd.viewangles,
                cmd.forwardmove,
                cmd.sidemove,
            );
            cmd.forwardmove = corrected_forward;
            cmd.sidemove = correct_side;
        }
    }
    pub fn bhop(&mut self, cmd: &mut UserCmd) -> OxideResult<()> {
        let p_local = Entity::get_local()?;
        let on_ground = p_local.get_flags().get(Flag::ONGROUND);
        let jumping = cmd.buttons.get(ButtonFlags::InJump);

        if !setting!(movement, bhop) {
            return Ok(());
        }

        cmd.buttons.set(ButtonFlags::InJump, on_ground && jumping);
        Ok(())
    }
    pub fn auto_strafe(&self, cmd: &mut UserCmd) -> OxideResult<()> {
        let p_local = Entity::get_local()?;
        if p_local.get_flags().get(Flag::ONGROUND) || !setting!(movement, autostrafe) {
            return Ok(());
        }
        let velocity = p_local.get_velocity();
        let speed = velocity.len2d();

        let air_accelerate = get_cvar("sv_airaccelerate".to_owned()).unwrap().float_value;

        let term = WISH_SPEED / air_accelerate / SPEED_VAR * 100.0 / speed;

        let perfect_delta = if -1.0 < term && term < 1.0 {
            term.acos()
        } else {
            0.0
        };

        let yaw = dtr(cmd.viewangles.yaw);
        let angle = velocity.y.atan2(velocity.x) - yaw;
        let desired_angle = (-cmd.sidemove).atan2(cmd.forwardmove);
        let mut delta = angle - desired_angle;
        while delta > PI {
            delta -= 2.0 * PI
        }
        while delta < -PI {
            delta += 2.0 * PI
        }

        let direction = if delta < 0.0 {
            angle + perfect_delta
        } else {
            angle - perfect_delta
        };

        cmd.forwardmove = direction.cos() * 450.0;
        cmd.sidemove = -direction.sin() * 450.0;
        Ok(())
    }
    pub fn calculate_correct_movement(
        &self,
        org_view_angles: Angles,
        new_view_angles: &Angles,
        old_forward: f32,
        old_side: f32,
    ) -> (f32, f32) {
        let alpha = (new_view_angles.yaw - org_view_angles.yaw) * PI / 180f32;

        let forward = old_forward * alpha.cos() - old_side * alpha.sin();
        let side = old_side * alpha.cos() + old_forward * alpha.sin();

        (forward, side)
    }
}
impl Cheat for Movement {
    fn handle_event(&mut self, _: &mut crate::draw::event::Event) {}
}
