use std::f32::consts::PI;

use crate::{
    error::OxideResult,
    math::{dtr, vector2::Vector2},
    o,
    sdk::{
        entity::{
            flags::Flag,
            player::{player_class::PlayerClass, Player},
            WaterLevel,
        },
        interfaces::cvar::get_cvar,
        user_cmd::{ButtonFlags, UserCmd},
    },
    setting, vmt_call,
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
        let p_local = Player::get_local()?;
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
    pub fn momentum_compensation(&mut self, cmd: &mut UserCmd) {
        if !setting!(movement, momentum_compensation) {
            return;
        }
        let p_local = Player::get_local().unwrap();

        if p_local.get_flags().get(Flag::ONGROUND) {}

        let vel = p_local.get_velocity().clone();
        if vel.len2d() == 0.0 {
            return;
        }
        let rotated_vel = self.rotate_movement(-cmd.viewangles.yaw + 180.0, Vector2::new(vel.x, vel.y));
        let drop = rotated_vel * *p_local.get_friction() * o!().global_vars.frametime;

        if rotated_vel.len() < drop.len() {
            return;
        }
        if cmd.forwardmove == 0.0 {
            cmd.forwardmove = rotated_vel.x - drop.x
        };
        if cmd.sidemove == 0.0 {
            cmd.sidemove = -rotated_vel.y + drop.y
        }
    }
    pub fn no_push(&mut self) {
        if setting!(movement, no_push) {
            let cvar = get_cvar("tf_avoidteammates_pushaway").unwrap();
            vmt_call!(cvar, set_int_value, 0);
        }
    }
    pub fn create_move_after(&mut self, cmd: &mut UserCmd, org_cmd: &UserCmd) {
        self.no_push();
        if org_cmd.viewangles.yaw != cmd.viewangles.yaw {
            let Vector2 { x, y } = self.rotate_movement(
                &org_cmd.viewangles.yaw - &cmd.viewangles.yaw,
                Vector2::new(cmd.forwardmove, cmd.sidemove),
            );
            cmd.forwardmove = x;
            cmd.sidemove = y;
        }
        self.momentum_compensation(cmd);
    }
    pub fn bhop(&mut self, cmd: &mut UserCmd) -> OxideResult<()> {
        let p_local = Player::get_local()?;
        let on_ground = p_local.get_flags().get(Flag::ONGROUND);
        let jumping = cmd.buttons.get(ButtonFlags::InJump);

        if !setting!(movement, bhop) {
            return Ok(());
        }

        cmd.buttons.set(ButtonFlags::InJump, on_ground && jumping);
        Ok(())
    }
    pub fn auto_strafe(&self, cmd: &mut UserCmd) -> OxideResult<()> {
        let p_local = Player::get_local()?;
        if p_local.get_flags().get(Flag::ONGROUND) || !setting!(movement, autostrafe) {
            return Ok(());
        }
        let velocity = p_local.get_velocity();
        let speed = velocity.len2d();

        let air_accelerate = get_cvar("sv_airaccelerate").unwrap().float_value;

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
    pub fn rotate_movement(&self, yaw: f32, vec: Vector2) -> Vector2 {
        let alpha = dtr(yaw);

        Vector2::new(
            vec.x * alpha.cos() - vec.y * alpha.sin(),
            vec.y * alpha.cos() + vec.x * alpha.sin(),
        )
    }
}
impl Cheat for Movement {
    fn handle_event(&mut self, _: &mut crate::draw::event::Event) {}
}
