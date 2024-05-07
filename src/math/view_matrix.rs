use std::mem::MaybeUninit;

use crate::{interface, math::vector2::Vector2, vmt_call};

use super::vector3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct VMatrix(pub [[f32; 4]; 4]);

impl VMatrix {
    pub fn default() -> VMatrix {
        let mut w2s = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut void = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe {
            let player_view = MaybeUninit::zeroed().assume_init();
            vmt_call!(interface!(base_client), get_player_view, &player_view);

            vmt_call!(
                interface!(render_view),
                get_matrices_for_view,
                &player_view,
                &mut void,
                &mut void,
                &mut w2s,
                &mut void
            );
        }
        w2s
    }
    pub fn w2s(&self, vec: &Vector3) -> Option<Vector2> {
        let screen_w = 0;
        let screen_h = 0;

        vmt_call!(
            interface!(base_engine),
            get_screen_size,
            &screen_w,
            &screen_h
        );
        let w = self.0[3][0] * vec.x + self.0[3][1] * vec.y + self.0[3][2] * vec.z + self.0[3][3];

        if w < 0.01 {
            return None;
        }

        let x = self.0[0][0] * vec.x + self.0[0][1] * vec.y + self.0[0][2] * vec.z + self.0[0][3];
        let y = self.0[1][0] * vec.x + self.0[1][1] * vec.y + self.0[1][2] * vec.z + self.0[1][3];

        Some(Vector2::new(
            screen_w as f32 / 2f32 * (1f32 + x / w),
            screen_h as f32 / 2f32 * (1f32 - y / w),
        ))
    }
}
