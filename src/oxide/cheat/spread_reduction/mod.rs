use crate::{
    math::vector::Vector2,
    o,
    sdk::{
        entity::{player::Player, weapon::Gun},
        interfaces::engine_trace::{trace, CONTENTS_GRATE, MASK_SHOT},
        user_cmd::{ButtonFlags, UserCmd},
    },
    setting, vmt_call,
};

use self::seed_prediction::State;

use super::{aimbot::Target, Cheat};

pub mod seed_prediction;

const SPREAD_CONCENTRATINO_AREA_SIZE: f32 = 20.0;

#[derive(Debug)]
pub struct SpreadReduction {
    pub playerperf_send_data: Option<(f32, f32)>,
    pub state: State,
}

impl SpreadReduction {
    pub fn name() -> &'static str {
        "SpreadReduction"
    }
    pub fn init() -> SpreadReduction {
        SpreadReduction {
            playerperf_send_data: None,
            state: State::UNSYNCED,
        }
    }
    pub fn should_run(&self) -> bool {
        setting!(aimbot, spread_reduction) && Player::get_local().is_ok()
    }
}

impl SpreadReduction {
    pub fn get_hit_cone(&mut self, target: Option<Target>, cmd: &UserCmd) -> f32 {
        let p_local = Player::get_local().unwrap();
        let src = vmt_call!(p_local.as_ent(), eye_position);
        let dist = if let Some(target) = target {
            (target.point - src).len()
        } else {
            let dir = cmd.viewangles.to_vectors().forward * 1000000.0;
            let trace = trace(src, src + dir, MASK_SHOT | CONTENTS_GRATE);
            let dist = (trace.endpos - trace.startpos).len();
            dist
        };
        SPREAD_CONCENTRATINO_AREA_SIZE.atan2(dist) * 2.0
    }
    pub fn can_tapfire(&mut self, gun: &mut Gun, spread_cone: f32, hit_cone: f32) -> bool {
        let bullet_count = gun.get_bullets();
        let last_shot = o!().global_vars.now() - *gun.as_weapon().get_last_fire();
        let fire_delay =
            gun.as_weapon().get_info().weapon_data[gun.as_weapon().get_mode()].time_fire_delay;

        let accuracy_delay = if bullet_count == 1 { 1.25 } else { 0.25 };
        if last_shot > accuracy_delay {
            return true;
        }
        if (hit_cone.powi(2) / spread_cone.powi(2))
            * bullet_count as f32
            * (1.0 / fire_delay)
            * accuracy_delay
            > 1.0
        {
            return true;
        }
        false
    }
    pub fn create_move(&mut self, cmd: &mut UserCmd, target: Option<Target>) {
        if !cmd.buttons.get(ButtonFlags::InAttack) {
            return;
        }
        let p_local = Player::get_local().unwrap();
        let weapon = vmt_call!(p_local.as_ent(), get_weapon);
        let Ok(gun) = weapon.as_gun() else { return };
        let spread_cone = vmt_call!(gun, get_projectile_spread);
        let hit_cone = self.get_hit_cone(target, cmd);
        let weapon_id = vmt_call!(gun.as_weapon(), get_weapon_id);
        if !matches!(self.state, State::SYNCED { .. }) {
            if setting!(aimbot, tapfire)
                && ((setting!(aimbot, tapfire_only_minigun)
                    && matches!(
                        weapon_id,
                        crate::sdk::entity::weapon::ids::WeaponType::Minigun
                    ))
                    || !setting!(aimbot, tapfire_only_minigun))
            {
                cmd.buttons.set(
                    ButtonFlags::InAttack,
                    self.can_tapfire(gun, spread_cone, hit_cone),
                );
            }
            return;
        }
        if !self.should_run() {
            return;
        }
        let State::SYNCED { last_seed,..}= &mut self.state else {unreachable!()};
        *last_seed = None;
        let bullets = self.calculate_bullet_trajectories(gun, spread_cone);

        let spread_correction = self.calculate_spread_correction(bullets, hit_cone);

        let mut normalized_angles = cmd.viewangles;
        normalized_angles.yaw += 180.0;
        normalized_angles.pitch = -normalized_angles.pitch;
        let dirs = normalized_angles.to_vectors();

        cmd.viewangles =
            (dirs.forward + dirs.right * spread_correction.x + dirs.up * spread_correction.y)
                .angle()
    }
    pub fn calculate_spread_correction(&self, mut bullets: Vec<Vector2>, hit_cone: f32) -> Vector2 {
        let mut best = (vec![], Vector2::zeroed());
        for _ in bullets.clone() {
            let mut points = vec![];
            for bullet in &bullets {
                points.push(*bullet);
                let mut avg = Vector2::zeroed();
                for point in &points {
                    avg += *point;
                }
                avg /= points.len() as f32;
                let mut all_hit = true;
                for bullet in &points {
                    if (*bullet - avg).len() > hit_cone {
                        all_hit = false;
                        break;
                    }
                }
                if all_hit {
                    if points.len() > best.0.len() {
                        best = (points.clone(), avg);
                    }
                } else {
                    points.pop();
                }
            }
            bullets.remove(0);
        }
        let mut closest_to_avg = best.0[0];
        for bullet in best.0 {
            if (bullet - best.1).len() < (closest_to_avg - best.1).len() {
                closest_to_avg = bullet
            }
        }
        closest_to_avg
    }
    pub fn calculate_bullet_trajectories(&mut self, gun: &mut Gun, spread: f32) -> Vec<Vector2> {
        let bullet_count = gun.get_bullets();
        let last_shot = o!().global_vars.now() - *gun.as_weapon().get_last_fire();

        let time = (o!().util.plat_float_time)() as f32;
        let server_time = self.get_server_time(time).unwrap();
        let seed = self.calculate_seed(server_time);

        let State::SYNCED { last_seed,..}= &mut self.state else {unreachable!()};

        *last_seed = Some(seed);

        let mut bullets = Vec::new();
        for i in 0..bullet_count {
            if i == 0
                && ((last_shot > 1.25 && bullet_count == 1)
                    || (last_shot > 0.25 && bullet_count > 1))
            {
                bullets.push(Vector2::new(0.0, 0.0));
            }
            (o!().util.random_seed)(seed + i);
            let yaw = (o!().util.random_float)(-0.5, 0.5) + (o!().util.random_float)(-0.5, 0.5);
            let pitch = (o!().util.random_float)(-0.5, 0.5) + (o!().util.random_float)(-0.5, 0.5);
            bullets.push(Vector2::new(yaw * spread, pitch * spread));
        }
        return bullets;
    }
}
impl Cheat for SpreadReduction {}
