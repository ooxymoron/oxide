use std::f32::consts::PI;

use crate::{
    d,
    draw::{colors::WHITE, frame::Frame},
    error::OxideResult,
    o,
    sdk::entity::{player::Player, Entity},
    setting, vmt_call,
};

use super::{Component, ComponentBase};

#[derive(Debug)]
pub struct AimbotFov {
    base: ComponentBase,
}

impl AimbotFov {
    pub fn new() -> AimbotFov {
        let size = d!().window_size;
        AimbotFov {
            base: ComponentBase {
                x: 0,
                y: 0,
                w: size.0,
                h: size.1,
            },
        }
    }
    fn should_draw(&self) -> bool {
        if !setting!(aimbot, enabled) || !setting!(aimbot, draw_fov) {
            return false;
        }
        let Ok(p_local) = Player::get_local() else {
            return false;
        };
        if !vmt_call!(p_local.as_ent(), is_alive) {
            return false;
        }
        if o!().fov <= setting!(aimbot, fov) {
            return false;
        }
        true
    }
}

impl Component for AimbotFov {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        let size = d!().window_size;
        self.base.w = size.0;
        self.base.h = size.1;
        if !self.should_draw() {
            return Ok(());
        }
        let size = d!().window_size;
        let aimbot_fov = setting!(aimbot, fov);
        let fov = o!().fov;

        let screen_fov = size.0 as f32 / size.1 as f32 / (4f32 / 3f32);
        let real_fov = (screen_fov * (fov / 360f32 * PI).tan()).atan();
        let radius = (aimbot_fov * PI / 360f32).tan() / (real_fov).tan() * size.0 as f32;

        frame.circle(size.0 / 2, size.1 / 2, radius, WHITE, 100);
        Ok(())
    }

    fn get_base(&mut self) -> &mut super::ComponentBase {
        todo!()
    }
}
