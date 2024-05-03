use std::{ffi::CString, mem::transmute};

use crate::{
    interface,
    math::angles::Angles,
    o,
    sdk::{
        bf_read::BfRead,
        entity::{player::Player, weapon::Gun},
        net_channel::{LatencyFlow, NetMessage, NetMessageTypeClient},
        user_cmd::{ButtonFlags, UserCmd},
    },
    vmt_call,
};

use super::Cheat;

const PLAYERPERF_COOLDOWN: f32 = 60.0;
const MIN_MANTISA: i32 = 13;

#[macro_export]
macro_rules! spread_prediction_log {
    ($($arg:tt)*) => {
        {
            let text = format!("<spread reduction> {}",format!($($arg)*));
            crate::o!().logger.log(&text);
        }
    };
}

#[derive(Debug, Clone)]
pub struct SpreadReduction {
    pub playerperf_send_data: Option<(f32, f32)>,
    pub delta: f32,
    pub last_predicted_time: f32,
    pub next_playerperf: f32,
}

impl SpreadReduction {
    pub fn name() -> &'static str {
        "SpreadReduction"
    }
    pub fn init() -> SpreadReduction {
        SpreadReduction {
            playerperf_send_data: None,
            delta: 1.0,
            last_predicted_time: 0.0,
            next_playerperf: 0.0,
        }
    }
    pub fn should_sync_delta(&self, message: &NetMessage) -> bool {
        matches!(message.get_type(), NetMessageTypeClient::Move)
            && self.playerperf_send_data.is_none()
            && self.next_playerperf < (o!().util.plat_float_time)() as f32
    }
    pub fn send_user_msg_post(&mut self) {
        let playerperf_cmd = CString::new("playerperf").unwrap();
        vmt_call!(
            interface!(base_engine),
            send_cmd_unrestricted,
            playerperf_cmd.as_ptr()
        );
        let latency = vmt_call!(
            interface!(base_engine).get_net_channel().unwrap(),
            get_latency,
            LatencyFlow::OUTGOING
        );
        self.playerperf_send_data = Some(((o!().util.plat_float_time)() as f32, latency));
    }

    pub fn dispatch_user_message(&mut self, msg_type: u32, buffer: &mut BfRead) -> bool {
        if msg_type != 5 {
            return false;
        }

        let destiantion = buffer.read_byte().unwrap();
        if destiantion != 2 {
            return false;
        }
        if self.playerperf_send_data.is_none() {
            return false;
        };

        let message = buffer.read_string(200).unwrap();
        let mut server_time_records = Vec::new();
        for record in message.split("\n") {
            let data = record.split(" ").collect::<Vec<_>>();
            if !(data.len() == 7 || data.len() == 3) {
                continue;
            }
            let Ok(float) = data[0].parse::<f32>() else {return false};

            server_time_records.push(float);
        }
        let Some(&server_time) = server_time_records.first() else {return false};
        //FIXME: temporary for testing
        let server_time = server_time + 2f32.powi(13);

        self.update_delta(server_time);
        self.playerperf_send_data = None;
        self.next_playerperf = (o!().util.plat_float_time)() as f32 + PLAYERPERF_COOLDOWN;
        return true;
    }
    //pub fn get_required_percision(&mut self, time: f32) {
    //    let mantisa = ((unsafe { transmute::<f32, i32>(time * 1000f32) } >> 23) & 0xff) - 127;
    //}
    pub fn update_delta(&mut self, server_time: f32) {
        //check float precision
        //9 is 0.01 percision
        //minimum server uptime is around 2 hours to achieve this percision
        let mantisa = ((unsafe { transmute::<_, i32>(server_time * 1000.0) } >> 23) & 0xff) - 127;
        if mantisa < MIN_MANTISA {
            spread_prediction_log!(
                "server too young to accurately predict seed\tmantisa: {}, required {}",
                mantisa,
                MIN_MANTISA
            );
            return;
        }

        let send_data = &self.playerperf_send_data.unwrap();
        let guess = send_data.0 + send_data.1 + self.delta;
        let error = guess - server_time;

        spread_prediction_log!(
            "resyncing server_time: {}\tguess_delta: {}\terror: {}\tnew_delta: {}\t ",
            server_time,
            self.delta,
            error,
            self.delta - error
        );
        self.delta -= error;
    }
    pub fn get_server_time(&self, client_time: f32) -> f32 {
        let latency = vmt_call!(
            interface!(base_engine).get_net_channel().unwrap(),
            get_latency,
            LatencyFlow::OUTGOING
        );
        client_time + self.delta + latency
    }
    pub fn get_seed(&self, time: f32) -> i32 {
        (unsafe { transmute::<_, i32>((time) * 1000.0) } & 0xFF)
    }
    pub fn create_move(&mut self, cmd: &UserCmd) {
        if !cmd.buttons.get(ButtonFlags::InAttack) {
            return;
        }
        let plocal = Player::get_local().unwrap();
        let weapon = vmt_call!(plocal.as_ent(), get_weapon);
        if let Ok(gun) = weapon.as_gun() {
            self.calculate_spread(gun);
        }
        let time = (o!().util.plat_float_time)() as f32;
        self.last_predicted_time = self.get_server_time(time);
    }
    pub fn calculate_spread(&mut self, gun: &mut Gun) {
        let spread = vmt_call!(gun, get_projectile_spread);
        if spread == 0.0 {
            return;
        }

        let mode = gun.as_weapon().get_mode();
        let mut bullet_count =
            gun.as_weapon().get_info().weapon_dat[mode as usize].bullets_per_shot;
        if let Some(bullets_attrib) = gun
            .as_weapon()
            .as_ent()
            .get_float_attrib("mult_bullets_per_shot")
        {
            bullet_count = bullets_attrib as i32
        }
        dbg!(bullet_count);
        let time = (o!().util.plat_float_time)() as f32;
        let seed = self.get_seed(self.get_server_time(time));
        let mut bullets = Vec::new();
        for i in 0..bullet_count {
            //RandomSeed(seed + i);
            (o!().util.random_seed)(seed + i);
            //float flX = RandomFloat(-0.5, 0.5) + RandomFloat(-0.5, 0.5);
            let yaw = (o!().util.random_float)(-0.5, 0.5) + (o!().util.random_float)(-0.5, 0.5);
            //float flY = RandomFloat(-0.5, 0.5) + RandomFloat(-0.5, 0.5);
            let pitch = (o!().util.random_float)(-0.5, 0.5) + (o!().util.random_float)(-0.5, 0.5);
            let directions = Angles::new(0.0, 0.0, 0.0).to_vectors();
            bullets.push(directions[0] + (directions[1] * yaw + directions[2] * pitch))
        }
        dbg!(bullets);
    }
}
impl Cheat for SpreadReduction {}