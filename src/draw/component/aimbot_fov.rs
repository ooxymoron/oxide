use std::f32::consts::PI;

use crate::{
    vmt_call, draw::{colors::YELLOW, frame::Frame}, error::OxideResult, o, sdk::entity::Entity, setting 
};

use super::Component;

#[derive(Debug)]
pub struct AimbotFov {}

impl AimbotFov {
    fn should_draw(&self) -> bool {
        if !setting!(aimbot,enabled) || !setting!(aimbot,draw_fov) {
            return false;
        }
        let Ok(p_local) = Entity::get_local() else {
            return false;
        };
        if !vmt_call!(p_local.as_ent(), is_alive) {
            return false;
        }
        true
    }
}

impl Component for AimbotFov {
    fn draw(&mut self, frame: &mut Frame, _: isize, _: isize) -> OxideResult<()>{
        if !self.should_draw() {
            return Ok(());
        }
        let size = frame.window_size();
        let aimbot_fov = setting!(aimbot,fov);
        let Some(fov) = o!().fov else {return Ok(())};

        let screen_fov = size.0 as f32 / size.1 as f32 / (4f32 / 3f32);
        let real_fov = (screen_fov * (fov / 360f32 * PI).tan()).atan();
        let radius = (aimbot_fov * PI / 360f32).tan() / (real_fov).tan() * size.0 as f32;

        frame.circle(size.0 / 2, size.1 / 2, radius, YELLOW, 200);
        Ok(())
    }

}
